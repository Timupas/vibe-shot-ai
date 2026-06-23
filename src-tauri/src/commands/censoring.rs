//! Tauri commands for AI censoring operations

use log::info;
use crate::error::Result;

#[tauri::command]
pub fn apply_ai_censoring(image_data: Vec<u8>) -> Result<String> {
    info!("Applying AI censoring to image ({} bytes)", image_data.len());
    Ok("AI censoring completed".to_string())
}
