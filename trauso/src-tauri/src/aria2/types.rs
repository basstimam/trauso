use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2Options {
    pub dir: Option<String>,
    #[serde(rename = "max-connection-per-server")]
    pub max_connection_per_server: Option<String>,
    pub split: Option<String>,
    #[serde(rename = "min-split-size")]
    pub min_split_size: Option<String>,
    #[serde(rename = "user-agent")]
    pub user_agent: Option<String>,
    pub out: Option<String>,
    pub header: Option<Vec<String>>,
    #[serde(rename = "file-allocation")]
    pub file_allocation: Option<String>,
    #[serde(rename = "continue")]
    pub continue_download: Option<String>,
}

impl Default for Aria2Options {
    fn default() -> Self {
        Self {
            dir: None,
            max_connection_per_server: Some("16".to_string()),
            split: Some("16".to_string()),
            min_split_size: Some("1M".to_string()),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()),
            out: None,
            header: None,
            file_allocation: Some("none".to_string()),
            continue_download: Some("true".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2RpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

impl Aria2RpcRequest {
    pub fn new(method: &str, params: Vec<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            method: format!("aria2.{}", method),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<T>,
    pub error: Option<Aria2Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2Error {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2Status {
    pub gid: String,
    pub status: String,
    #[serde(rename = "totalLength")]
    pub total_length: Option<String>,
    #[serde(rename = "completedLength")]
    pub completed_length: Option<String>,
    #[serde(rename = "downloadSpeed")]
    pub download_speed: Option<String>,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: Option<String>,
    pub connections: Option<String>,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    pub files: Option<Vec<Aria2File>>,
    pub dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2File {
    pub index: String,
    pub path: String,
    pub length: String,
    #[serde(rename = "completedLength")]
    pub completed_length: String,
    pub selected: String,
    pub uris: Option<Vec<Aria2Uri>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2Uri {
    pub uri: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub gid: String,
    pub filename: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub speed: u64,
    pub progress: f64,
    pub status: DownloadStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Active,
    Waiting,
    Paused,
    Complete,
    Error,
    Removed,
}

impl From<&str> for DownloadStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => DownloadStatus::Active,
            "waiting" => DownloadStatus::Waiting,
            "paused" => DownloadStatus::Paused,
            "complete" => DownloadStatus::Complete,
            "error" => DownloadStatus::Error,
            "removed" => DownloadStatus::Removed,
            _ => DownloadStatus::Error,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aria2GlobalStat {
    #[serde(rename = "downloadSpeed")]
    pub download_speed: String,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: String,
    #[serde(rename = "numActive")]
    pub num_active: String,
    #[serde(rename = "numWaiting")]
    pub num_waiting: String,
    #[serde(rename = "numStopped")]
    pub num_stopped: String,
    #[serde(rename = "numStoppedTotal")]
    pub num_stopped_total: String,
}
