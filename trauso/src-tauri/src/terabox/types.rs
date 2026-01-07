use serde::{Deserialize, Serialize};

/// File info returned to frontend (computed fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeraboxFileInfo {
    pub is_dir: bool,
    pub fs_id: String,
    pub name: String,
    pub file_type: String,
    pub size: Option<i64>,
    pub category: Option<String>,
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeraboxInfo {
    pub ok: bool,
    pub shareid: i64,
    pub uk: i64,
    pub sign: String,
    pub timestamp: i64,
    #[serde(default)]
    pub list: Vec<TeraboxFileInfo>,
    #[serde(default)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadParams {
    pub shareid: i64,
    pub uk: i64,
    pub sign: String,
    pub timestamp: i64,
    pub fs_id: String,
    #[serde(default = "default_mode")]
    pub mode: i32, // 1 for Server 1, 2 for Server 2
}

fn default_mode() -> i32 {
    2 // Default to Server 2 (get-downloadp) as it seems more stable/common
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    pub ok: bool,
    pub download_link: Option<String>,
    pub error_message: Option<String>,
}

// ============ API Response Types (raw from worker) ============

/// Raw response from /api/get-info-new
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfoResponse {
    pub ok: bool,
    pub shareid: Option<i64>,
    pub uk: Option<i64>,
    pub sign: Option<String>,
    pub timestamp: Option<i64>,
    pub list: Option<Vec<WorkerFileItem>>,
    pub message: Option<String>,
}

/// Raw file item from API (loosely typed to handle inconsistencies)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerFileItem {
    pub category: Option<String>,
    
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub fs_id: String,
    
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub is_dir: String,
    
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub size: String,
    
    pub filename: String,
    pub create_time: Option<String>,
}

/// Raw response from /api/get-download or /api/get-downloadp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerDownloadResponse {
    pub ok: bool,
    pub retry: Option<bool>,
    #[serde(rename = "downloadLink")]
    pub download_link: Option<String>,
    pub message: Option<String>,
}

fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("Expected string or number")),
    }
}
