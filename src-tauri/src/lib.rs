mod aria2;
mod terabox;

use aria2::{Aria2Client, Aria2Options, DownloadInfo};
use terabox::{DownloadLink, DownloadParams, TeraboxApi, TeraboxInfo};
use std::sync::LazyLock;
use tokio::sync::Mutex;

static TERABOX_API: LazyLock<TeraboxApi> = LazyLock::new(TeraboxApi::new);
static ARIA2_CLIENT: LazyLock<Mutex<Aria2Client>> = LazyLock::new(|| Mutex::new(Aria2Client::default()));

#[tauri::command]
async fn get_terabox_info(url: String) -> Result<TeraboxInfo, String> {
    TERABOX_API.get_info(&url).await
}

#[tauri::command]
async fn get_download_link(params: DownloadParams) -> Result<DownloadLink, String> {
    TERABOX_API.get_download_link(params).await
}

#[tauri::command]
fn extract_shorturl(url: String) -> Option<String> {
    TeraboxApi::extract_shorturl(&url)
}

#[tauri::command]
async fn start_aria2() -> Result<(), String> {
    let client = ARIA2_CLIENT.lock().await;
    client.start_daemon().await
}

#[tauri::command]
async fn stop_aria2() -> Result<(), String> {
    let client = ARIA2_CLIENT.lock().await;
    client.stop_daemon().await
}

#[tauri::command]
async fn is_aria2_running() -> bool {
    let client = ARIA2_CLIENT.lock().await;
    client.is_running().await
}

#[tauri::command]
async fn add_download(url: String, dir: Option<String>, filename: Option<String>) -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    
    let options = Aria2Options {
        dir,
        out: filename,
        ..Default::default()
    };
    
    client.add_uri(&url, Some(options)).await
}

#[tauri::command]
async fn get_download_status(gid: String) -> Result<DownloadInfo, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.get_download_info(&gid).await
}

#[tauri::command]
async fn pause_download(gid: String) -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.pause(&gid).await
}

#[tauri::command]
async fn resume_download(gid: String) -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.unpause(&gid).await
}

#[tauri::command]
async fn cancel_download(gid: String) -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.force_remove(&gid).await
}

#[tauri::command]
async fn get_all_downloads() -> Result<Vec<DownloadInfo>, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.get_all_downloads().await
}

#[tauri::command]
async fn pause_all_downloads() -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.pause_all().await
}

#[tauri::command]
async fn resume_all_downloads() -> Result<String, String> {
    let client = ARIA2_CLIENT.lock().await;
    client.unpause_all().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_terabox_info,
            get_download_link,
            extract_shorturl,
            start_aria2,
            stop_aria2,
            is_aria2_running,
            add_download,
            get_download_status,
            pause_download,
            resume_download,
            cancel_download,
            get_all_downloads,
            pause_all_downloads,
            resume_all_downloads,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
