//! Global hotkey management module
//! 
//! Handles registration and triggering of global hotkeys for screenshot capture.

use crate::error::{Result, VibeError};
use log::{info, warn};
use tauri::{AppHandle, GlobalShortcutManager};

/// Register global hotkeys for the application
pub fn register_hotkeys(app: &AppHandle) -> Result<()> {
    info!("Registering global hotkeys");
    
    let mut shortcut_manager = app.global_shortcut_manager();
    
    // Register Alt+Shift+S for screenshot
    const SCREENSHOT_HOTKEY: &str = "Alt+Shift+S";
    
    match shortcut_manager.register(SCREENSHOT_HOTKEY, move || {
        info!("Screenshot hotkey triggered: {}", SCREENSHOT_HOTKEY);
        // The actual screenshot handling will be done in the frontend command
    }) {
        Ok(_) => {
            info!("Successfully registered hotkey: {}", SCREENSHOT_HOTKEY);
            Ok(())
        }
        Err(e) => {
            warn!("Failed to register hotkey {}: {}", SCREENSHOT_HOTKEY, e);
            Err(VibeError::HotkeyError(format!(
                "Failed to register hotkey {}: {}",
                SCREENSHOT_HOTKEY, e
            )))
        }
    }
}

/// Unregister all global hotkeys
pub fn unregister_hotkeys(app: &AppHandle) -> Result<()> {
    info!("Unregistering global hotkeys");
    let mut shortcut_manager = app.global_shortcut_manager();
    
    if let Err(e) = shortcut_manager.unregister_all() {
        warn!("Failed to unregister all hotkeys: {}", e);
        return Err(VibeError::HotkeyError(format!(
            "Failed to unregister all hotkeys: {}",
            e
        )));
    }
    
    Ok(())
}
