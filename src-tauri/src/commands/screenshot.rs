//! Tauri commands for screenshot operations

use log::info;
use crate::error::Result;

#[tauri::command]
pub fn capture_area(x: i32, y: i32, width: u32, height: u32) -> Result<String> {
    info!(
        "Capturing area: x={}, y={}, width={}, height={}",
        x, y, width, height
    );
    Ok("Area captured".to_string())
}

#[tauri::command]
pub fn save_screenshot(filepath: String, image_data: Vec<u8>) -> Result<String> {
    info!("Saving screenshot to: {}", filepath);
    Ok(format!("Screenshot saved to {}", filepath))
}
