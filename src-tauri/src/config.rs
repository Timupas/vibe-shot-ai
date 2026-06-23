use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: HotkeyConfig,
    pub screenshot: ScreenshotConfig,
    pub ai_censoring: AiCensoringConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Global hotkey for triggering screenshot (e.g., "Alt+Shift+S")
    pub screenshot_hotkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotConfig {
    /// Format for saved screenshots (png, jpg, bmp)
    pub format: String,
    /// Overlay fade duration in milliseconds
    pub overlay_fade_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCensoringConfig {
    /// Enable AI-powered censoring
    pub enabled: bool,
    /// Sensitivity level (low, medium, high)
    pub sensitivity: String,
    /// API endpoint for external AI (optional, for future cloud features)
    pub api_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Path to store screenshots
    pub screenshot_dir: PathBuf,
    /// Path to store application data
    pub app_data_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig {
                screenshot_hotkey: "Alt+Shift+S".to_string(),
            },
            screenshot: ScreenshotConfig {
                format: "png".to_string(),
                overlay_fade_ms: 150,
            },
            ai_censoring: AiCensoringConfig {
                enabled: true,
                sensitivity: "medium".to_string(),
                api_endpoint: None,
            },
            storage: StorageConfig {
                screenshot_dir: PathBuf::from("$APPDATA/vibeshot/screenshots"),
                app_data_dir: PathBuf::from("$APPDATA/vibeshot"),
            },
        }
    }
}
