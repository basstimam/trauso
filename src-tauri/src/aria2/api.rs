use crate::aria2::types::*;
use reqwest::Client;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

const DEFAULT_RPC_URL: &str = "http://localhost:6800/jsonrpc";
const ARIA2_STARTUP_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Aria2Client {
    client: Client,
    rpc_url: String,
    aria2_process: Mutex<Option<Child>>,
}

impl Default for Aria2Client {
    fn default() -> Self {
        Self::new(DEFAULT_RPC_URL)
    }
}

impl Aria2Client {
    pub fn new(rpc_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            rpc_url: rpc_url.to_string(),
            aria2_process: Mutex::new(None),
        }
    }

    fn get_aria2_path() -> Option<PathBuf> {
        let possible_paths = [
            PathBuf::from("aria2/aria2c.exe"),
            PathBuf::from("../aria2/aria2c.exe"),
            PathBuf::from("../../aria2/aria2c.exe"),
            PathBuf::from("_internal/aria2/aria2c.exe"),
        ];

        for path in &possible_paths {
            if path.exists() {
                return Some(path.clone());
            }
        }

        if let Ok(output) = Command::new("aria2c").arg("--version").output() {
            if output.status.success() {
                return Some(PathBuf::from("aria2c"));
            }
        }

        None
    }

    pub async fn start_daemon(&self) -> Result<(), String> {
        if self.is_running().await {
            return Ok(());
        }

        let aria2_path = Self::get_aria2_path().ok_or("aria2c not found")?;

        let args = [
            "--enable-rpc",
            "--rpc-listen-all=false",
            "--rpc-listen-port=6800",
            "--max-concurrent-downloads=5",
            "--max-connection-per-server=16",
            "--split=16",
            "--min-split-size=1M",
            "--max-overall-download-limit=0",
            "--max-download-limit=0",
            "--file-allocation=none",
            "--continue=true",
            "--auto-file-renaming=true",
            "--allow-overwrite=false",
        ];

        let mut cmd = Command::new(&aria2_path);
        cmd.args(&args)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let child = cmd.spawn()
            .map_err(|e| format!("Failed to start aria2c: {}", e))?;

        *self.aria2_process.lock().unwrap() = Some(child);

        let start = std::time::Instant::now();
        while start.elapsed() < ARIA2_STARTUP_TIMEOUT {
            if self.is_running().await {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        Err("aria2c failed to start within timeout".to_string())
    }

    pub async fn stop_daemon(&self) -> Result<(), String> {
        if let Some(mut child) = self.aria2_process.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }

        let _ = self.shutdown().await;
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        self.get_version().await.is_ok()
    }

    async fn call<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<T, String> {
        let request = Aria2RpcRequest::new(method, params);

        let response = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("RPC request failed: {}", e))?;

        let rpc_response: Aria2RpcResponse<T> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse RPC response: {}", e))?;

        if let Some(error) = rpc_response.error {
            return Err(format!("aria2 error: {} (code: {})", error.message, error.code));
        }

        rpc_response.result.ok_or("Empty response from aria2".to_string())
    }

    pub async fn get_version(&self) -> Result<serde_json::Value, String> {
        self.call("getVersion", vec![]).await
    }

    pub async fn add_uri(
        &self,
        url: &str,
        options: Option<Aria2Options>,
    ) -> Result<String, String> {
        let uris = serde_json::json!([url]);
        let opts = options.unwrap_or_default();
        let opts_json = serde_json::to_value(&opts).unwrap_or(serde_json::json!({}));

        self.call("addUri", vec![uris, opts_json]).await
    }

    pub async fn get_status(&self, gid: &str) -> Result<Aria2Status, String> {
        self.call("tellStatus", vec![serde_json::json!(gid)]).await
    }

    pub async fn get_download_info(&self, gid: &str) -> Result<DownloadInfo, String> {
        let status = self.get_status(gid).await?;

        let total_size: u64 = status.total_length.as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let downloaded: u64 = status.completed_length.as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let speed: u64 = status.download_speed.as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let progress = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        let filename = status.files
            .and_then(|files| files.first().cloned())
            .map(|f| {
                PathBuf::from(&f.path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| f.path.clone())
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(DownloadInfo {
            gid: status.gid,
            filename,
            total_size,
            downloaded,
            speed,
            progress,
            status: DownloadStatus::from(status.status.as_str()),
            error_message: status.error_message,
        })
    }

    pub async fn pause(&self, gid: &str) -> Result<String, String> {
        self.call("pause", vec![serde_json::json!(gid)]).await
    }

    pub async fn unpause(&self, gid: &str) -> Result<String, String> {
        self.call("unpause", vec![serde_json::json!(gid)]).await
    }

    pub async fn remove(&self, gid: &str) -> Result<String, String> {
        self.call("remove", vec![serde_json::json!(gid)]).await
    }

    pub async fn force_remove(&self, gid: &str) -> Result<String, String> {
        self.call("forceRemove", vec![serde_json::json!(gid)]).await
    }

    pub async fn pause_all(&self) -> Result<String, String> {
        self.call("pauseAll", vec![]).await
    }

    pub async fn unpause_all(&self) -> Result<String, String> {
        self.call("unpauseAll", vec![]).await
    }

    pub async fn get_global_stat(&self) -> Result<Aria2GlobalStat, String> {
        self.call("getGlobalStat", vec![]).await
    }

    pub async fn tell_active(&self) -> Result<Vec<Aria2Status>, String> {
        self.call("tellActive", vec![]).await
    }

    pub async fn tell_waiting(&self, offset: i32, num: i32) -> Result<Vec<Aria2Status>, String> {
        self.call("tellWaiting", vec![
            serde_json::json!(offset),
            serde_json::json!(num),
        ]).await
    }

    pub async fn tell_stopped(&self, offset: i32, num: i32) -> Result<Vec<Aria2Status>, String> {
        self.call("tellStopped", vec![
            serde_json::json!(offset),
            serde_json::json!(num),
        ]).await
    }

    pub async fn purge_download_result(&self) -> Result<String, String> {
        self.call("purgeDownloadResult", vec![]).await
    }

    pub async fn shutdown(&self) -> Result<String, String> {
        self.call("shutdown", vec![]).await
    }

    pub async fn get_all_downloads(&self) -> Result<Vec<DownloadInfo>, String> {
        let mut all_downloads = Vec::new();

        if let Ok(active) = self.tell_active().await {
            for status in active {
                if let Ok(info) = self.get_download_info(&status.gid).await {
                    all_downloads.push(info);
                }
            }
        }

        if let Ok(waiting) = self.tell_waiting(0, 100).await {
            for status in waiting {
                if let Ok(info) = self.get_download_info(&status.gid).await {
                    all_downloads.push(info);
                }
            }
        }

        if let Ok(stopped) = self.tell_stopped(0, 100).await {
            for status in stopped {
                if let Ok(info) = self.get_download_info(&status.gid).await {
                    all_downloads.push(info);
                }
            }
        }

        Ok(all_downloads)
    }
}

impl Drop for Aria2Client {
    fn drop(&mut self) {
        if let Some(mut child) = self.aria2_process.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
