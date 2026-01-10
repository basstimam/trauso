mod aria2;
mod settings;
mod terabox;

use aria2::{Aria2Client, Aria2Options, DownloadInfo};
use settings::types::AppSettings;
use terabox::{DownloadLink, DownloadParams, TeraboxApi, TeraboxInfo};
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tauri_plugin_store::StoreExt;

static TERABOX_API: LazyLock<TeraboxApi> = LazyLock::new(TeraboxApi::new);
static ARIA2_CLIENT: LazyLock<Mutex<Aria2Client>> = LazyLock::new(|| {
    Mutex::new(Aria2Client::new(
        "http://localhost:6800/jsonrpc",
        0,
        0,
    ))
});

fn get_settings(handle: &tauri::AppHandle) -> AppSettings {
    let store = handle.store("settings").unwrap();
    let result = store.get("app_settings").unwrap();
    serde_json::from_value(result).unwrap_or(AppSettings::default())
}

fn save_settings(handle: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    let store = handle.store("settings").unwrap();
    let value = serde_json::to_value(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    store.set("app_settings", value);
    Ok(())
}

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

#[tauri::command]
async fn set_bandwidth_limit(
    handle: tauri::AppHandle,
    max_overall_limit_kb_per_sec: u64,
    max_download_limit_kb_per_sec: u64,
) -> Result<(), String> {
    let was_running = {
        let client = ARIA2_CLIENT.lock().await;
        client.is_running().await
    };

    if was_running {
        let client = ARIA2_CLIENT.lock().await;
        client.stop_daemon().await?;
    }

    let client = ARIA2_CLIENT.lock().await;
    client.set_bandwidth_limit(max_overall_limit_kb_per_sec, max_download_limit_kb_per_sec);

    let mut settings = get_settings(&handle);
    settings.max_overall_download_limit_kb_per_sec = max_overall_limit_kb_per_sec;
    settings.max_download_limit_kb_per_sec = max_download_limit_kb_per_sec;
    save_settings(&handle, &settings)?;

    if was_running {
        client.start_daemon().await?;
    }

    Ok(())
}

#[tauri::command]
async fn get_bandwidth_limit(handle: tauri::AppHandle) -> (u64, u64) {
    let settings = get_settings(&handle);
    let client = ARIA2_CLIENT.lock().await;
    client.set_bandwidth_limit(
        settings.max_overall_download_limit_kb_per_sec,
        settings.max_download_limit_kb_per_sec,
    );
    client.get_bandwidth_limit()
}

#[tauri::command]
async fn get_app_settings(handle: tauri::AppHandle) -> Result<AppSettings, String> {
    Ok(get_settings(&handle))
}

#[tauri::command]
async fn save_app_settings(handle: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    save_settings(&handle, &settings)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
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
            set_bandwidth_limit,
            get_bandwidth_limit,
            get_app_settings,
            save_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
