use crate::settings::types::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static SETTINGS: Mutex<Option<AppSettings>> = Mutex::new(None);
static HISTORY: Mutex<Option<DownloadHistory>> = Mutex::new(None);

fn get_config_dir() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("trauso");
    fs::create_dir_all(&config_dir).ok();
    config_dir
}

fn get_settings_path() -> PathBuf {
    get_config_dir().join("settings.json")
}

fn get_history_path() -> PathBuf {
    get_config_dir().join("history.json")
}

pub fn load_settings() -> AppSettings {
    let mut settings_guard = SETTINGS.lock().unwrap();
    
    if let Some(ref settings) = *settings_guard {
        return settings.clone();
    }

    let path = get_settings_path();
    let settings = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        AppSettings::default()
    };

    *settings_guard = Some(settings.clone());
    settings
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_path();
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    let mut settings_guard = SETTINGS.lock().unwrap();
    *settings_guard = Some(settings.clone());

    Ok(())
}

pub fn load_history() -> DownloadHistory {
    let mut history_guard = HISTORY.lock().unwrap();
    
    if let Some(ref history) = *history_guard {
        return history.clone();
    }

    let path = get_history_path();
    let history = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        DownloadHistory::default()
    };

    *history_guard = Some(history.clone());
    history
}

pub fn save_history(history: &DownloadHistory) -> Result<(), String> {
    let path = get_history_path();
    let content = serde_json::to_string_pretty(history)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write history: {}", e))?;

    let mut history_guard = HISTORY.lock().unwrap();
    *history_guard = Some(history.clone());

    Ok(())
}

pub fn add_history_item(item: DownloadHistoryItem) -> Result<(), String> {
    let mut history = load_history();
    history.items.insert(0, item);
    
    if history.items.len() > 100 {
        history.items.truncate(100);
    }
    
    save_history(&history)
}

pub fn clear_history() -> Result<(), String> {
    save_history(&DownloadHistory::default())
}
