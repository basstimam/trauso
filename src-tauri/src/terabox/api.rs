use crate::terabox::types::*;
use regex::Regex;
use reqwest::Client;
use std::sync::LazyLock;
use std::time::Duration;

const BASE_URL: &str = "https://terabox.hnn.workers.dev";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36";

static SHORTURL_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"terabox\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"1024tera\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"1024terabox\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"4funbox\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"mirrobox\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"teraboxapp\.com/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"terabox\.app/s/([^/?\u{0026}]+)").unwrap(),
        Regex::new(r"surl=([^\u{0026}]+)").unwrap(),
        Regex::new(r"/s/([^/?\u{0026}]+)").unwrap(),
    ]
});

static SHORTURL_DIRECT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]{10,25}$").unwrap());

pub struct TeraboxApi {
    client: Client,
}

impl Default for TeraboxApi {
    fn default() -> Self {
        Self::new()
    }
}

impl TeraboxApi {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(USER_AGENT)
            .gzip(true)
            .deflate(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub fn extract_shorturl(url: &str) -> Option<String> {
        for pattern in SHORTURL_PATTERNS.iter() {
            if let Some(caps) = pattern.captures(url) {
                if let Some(m) = caps.get(1) {
                    return Some(m.as_str().to_string());
                }
            }
        }

        if SHORTURL_DIRECT.is_match(url) {
            return Some(url.to_string());
        }

        None
    }

    fn get_headers() -> Vec<(&'static str, String)> {
        vec![
            ("Accept", "*/*".to_string()),
            ("Accept-Language", "id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7".to_string()),
            ("Cache-Control", "no-cache".to_string()),
            ("Pragma", "no-cache".to_string()),
            ("Sec-Fetch-Site", "same-origin".to_string()),
            ("Sec-Fetch-Mode", "cors".to_string()),
            ("Sec-Fetch-Dest", "empty".to_string()),
            (
                "sec-ch-ua",
                r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#.to_string(),
            ),
            ("sec-ch-ua-mobile", "?0".to_string()),
            ("sec-ch-ua-platform", r#""Windows""#.to_string()),
            ("Priority", "u=1, i".to_string()),
            ("Referer", format!("{}/", BASE_URL)),
            ("Origin", BASE_URL.to_string()),
        ]
    }

    pub async fn get_info(&self, url: &str) -> Result<TeraboxInfo, String> {
        let shorturl = Self::extract_shorturl(url).ok_or("Invalid TeraBox URL")?;
        println!("Extracted shorturl: {}", shorturl);
        
        let headers = Self::get_headers();
        
        // Try get-info-new first, then fallback to get-info
        let endpoints = ["/api/get-info-new", "/api/get-info"];
        
        let mut last_error = String::from("Unknown error");

        for api_endpoint in &endpoints {
            let request_url = format!("{}{}", BASE_URL, api_endpoint);
            
            let mut request = self
                .client
                .get(&request_url)
                .query(&[("shorturl", &shorturl), ("pwd", &"".to_string())]);

            for (key, value) in &headers {
                request = request.header(*key, value);
            }

            match request.send().await {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        last_error = format!("Server returned error status: {}", status);
                        continue;
                    }

                    // Read text as String first
                    let text = match response.text().await {
                        Ok(t) => t,
                        Err(e) => {
                            last_error = format!("Failed to read response: {}", e);
                            continue;
                        }
                    };
                    
                    println!("Info API Response ({}): {}", api_endpoint, text);

                    match serde_json::from_str::<WorkerInfoResponse>(&text) {
                        Ok(data) => {
                            if data.ok {
                                let list = data
                                    .list
                                    .unwrap_or_default()
                                    .into_iter()
                                    .map(|item| Self::convert_file_item(item))
                                    .collect();

                                return Ok(TeraboxInfo {
                                    ok: true,
                                    shareid: data.shareid.unwrap_or(0),
                                    uk: data.uk.unwrap_or(0),
                                    sign: data.sign.unwrap_or_default(),
                                    timestamp: data.timestamp.unwrap_or(0),
                                    list,
                                    error_message: None,
                                });
                            } else {
                                last_error = data.message.unwrap_or("API returned ok=false".to_string());
                            }
                        }
                        Err(e) => {
                            last_error = format!("Failed to parse JSON: {}. Response len: {}", e, text.len());
                        }
                    }
                }
                Err(e) => {
                    last_error = format!("Request failed: {}", e);
                }
            }
        }

        Err(format!("Failed to get info: {}", last_error))
    }

    pub async fn get_download_link(&self, params: DownloadParams) -> Result<DownloadLink, String> {
        let request_body = serde_json::json!({
            "shareid": params.shareid,
            "uk": params.uk,
            "sign": params.sign,
            "timestamp": params.timestamp,
            "fs_id": params.fs_id,
        });

        let headers = Self::get_headers();
        
        // Determine primary and fallback endpoints based on selected mode
        let (primary, fallback) = if params.mode == 1 {
            ("/api/get-download", "/api/get-downloadp")
        } else {
            ("/api/get-downloadp", "/api/get-download") // Default Mode 2
        };

        let endpoints = [primary, fallback];
        let mut last_error = String::from("Unknown error");

        for endpoint in endpoints {
            let request_url = format!("{}{}", BASE_URL, endpoint);

            let mut request = self
                .client
                .post(&request_url)
                .header("Content-Type", "application/json")
                .json(&request_body);

            for (key, value) in &headers {
                request = request.header(*key, value);
            }

            println!("Trying Download Endpoint: {}", endpoint);

            match request.send().await {
                Ok(response) => {
                    let text = match response.text().await {
                        Ok(t) => t,
                        Err(e) => {
                            last_error = format!("Failed to read response from {}: {}", endpoint, e);
                            log::warn!("{}", last_error);
                            continue;
                        }
                    };
                    
                    println!("Download API Response ({}): {}", endpoint, text);

                    match serde_json::from_str::<WorkerDownloadResponse>(&text) {
                        Ok(data) => {
                            if data.ok && data.download_link.is_some() {
                                return Ok(DownloadLink {
                                    ok: true,
                                    download_link: data.download_link,
                                    error_message: None,
                                });
                            }
                            
                            // If API returns specific message, update error but try next server
                            if let Some(msg) = data.message {
                                last_error = format!("Server {} error: {}", endpoint, msg);
                            } else {
                                last_error = format!("Server {} returned ok=false", endpoint);
                            }
                        }
                        Err(e) => {
                            last_error = format!("Failed to parse JSON from {}: {}", endpoint, e);
                        }
                    }
                }
                Err(e) => {
                    last_error = format!("Request failed for {}: {}", request_url, e);
                }
            }
            
            println!("Retry: Switching to fallback server due to: {}", last_error);
        }

        Err(format!("All download servers failed. Last error: {}", last_error))
    }

    fn convert_file_item(item: WorkerFileItem) -> TeraboxFileInfo {
        let is_dir = item.is_dir == "1";
        let size: Option<i64> = if is_dir {
            None
        } else {
            item.size.parse().ok()
        };
        // create_time is already Option<String>, try parse if exists
        let create_time: Option<i64> = item.create_time.and_then(|t| t.parse().ok());
        
        let file_type = if is_dir {
            "folder".to_string()
        } else {
            Self::check_file_type(&item.filename)
        };

        TeraboxFileInfo {
            is_dir,
            fs_id: item.fs_id,
            name: item.filename,
            file_type,
            size,
            category: item.category,
            create_time,
        }
    }

    fn check_file_type(name: &str) -> String {
        let name_lower = name.to_lowercase();
        let video_exts = [
            ".mp4", ".mov", ".m4v", ".mkv", ".asf", ".avi", ".wmv", ".m2ts", ".3g2",
        ];
        let image_exts = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".svg"];
        let file_exts = [".pdf", ".docx", ".zip", ".rar", ".7z"];

        if video_exts.iter().any(|ext| name_lower.ends_with(ext)) {
            "video".to_string()
        } else if image_exts.iter().any(|ext| name_lower.ends_with(ext)) {
            "image".to_string()
        } else if file_exts.iter().any(|ext| name_lower.ends_with(ext)) {
            "file".to_string()
        } else {
            "other".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_shorturl() {
        assert_eq!(
            TeraboxApi::extract_shorturl("https://terabox.com/s/1DcGWQPuMVDgkXrFhP7AlcQ"),
            Some("1DcGWQPuMVDgkXrFhP7AlcQ".to_string())
        );
    }
}
