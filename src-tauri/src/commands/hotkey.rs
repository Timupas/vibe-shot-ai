//! Tauri commands for hotkey handling

use log::info;

#[tauri::command]
pub fn trigger_screenshot() -> String {
    info!("Screenshot triggered via command");
    "Screenshot mode activated".to_string()
}
