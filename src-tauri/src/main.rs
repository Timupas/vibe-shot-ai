// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod config;
mod error;

use log::{info, error};
use tauri::{GlobalShortcutManager, Manager};

fn main() {
    env_logger::init();
    info!("Starting VibeShot application");

    tauri::Builder::default()
        .setup(|app| {
            info!("Setting up VibeShot");
            
            // Initialize modules
            if let Err(e) = modules::hotkey::register_hotkeys(app) {
                error!("Failed to register hotkeys: {}", e);
            }
            
            if let Err(e) = modules::database::init_database(app) {
                error!("Failed to initialize database: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::hotkey::trigger_screenshot,
            commands::screenshot::capture_area,
            commands::censoring::apply_ai_censoring,
            commands::screenshot::save_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
