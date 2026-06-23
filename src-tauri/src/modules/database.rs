//! Database module for managing screenshot history and application settings
//!
//! Uses SQLite for local persistence. All sensitive data (API tokens, etc.) is encrypted.

use crate::error::{Result, VibeError};
use log::{info, error};
use rusqlite::{Connection, params};
use tauri::AppHandle;
use std::path::PathBuf;
use chrono::Utc;

/// Initialize the SQLite database
pub fn init_database(app: &AppHandle) -> Result<()> {
    info!("Initializing database");
    
    let db_path = get_database_path(app)?;
    
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let conn = Connection::open(&db_path)
        .map_err(|e| VibeError::DatabaseError(e))?;
    
    // Create tables
    create_tables(&conn)?;
    
    info!("Database initialized successfully at {:?}", db_path);
    Ok(())
}

/// Create necessary database tables
fn create_tables(conn: &Connection) -> Result<()> {
    info!("Creating database tables");
    
    // Screenshot history table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            filepath TEXT NOT NULL,
            thumbnail BLOB,
            created_at TEXT NOT NULL,
            filesize INTEGER,
            censoring_applied BOOLEAN DEFAULT 0,
            censoring_data TEXT
        )",
        [],
    ).map_err(|e| VibeError::DatabaseError(e))?;
    
    // Censored regions tracking
    conn.execute(
        "CREATE TABLE IF NOT EXISTS censored_regions (
            id TEXT PRIMARY KEY,
            screenshot_id TEXT NOT NULL,
            region_type TEXT NOT NULL,
            x INTEGER,
            y INTEGER,
            width INTEGER,
            height INTEGER,
            content_hash TEXT,
            manual_override BOOLEAN DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY(screenshot_id) REFERENCES screenshots(id)
        )",
        [],
    ).map_err(|e| VibeError::DatabaseError(e))?;
    
    // Application settings
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            encrypted BOOLEAN DEFAULT 0,
            updated_at TEXT NOT NULL
        )",
        [],
    ).map_err(|e| VibeError::DatabaseError(e))?;
    
    info!("Database tables created successfully");
    Ok(())
}

/// Get the database file path
fn get_database_path(app: &AppHandle) -> Result<PathBuf> {
    let app_data_dir = app.path_resolver()
        .app_data_dir()
        .ok_or_else(|| VibeError::ConfigError("Could not determine app data directory".to_string()))?;
    
    Ok(app_data_dir.join("vibeshot.db"))
}

/// Store a screenshot record in the database
pub fn store_screenshot(
    app: &AppHandle,
    filename: &str,
    filepath: &str,
    filesize: i64,
) -> Result<String> {
    info!("Storing screenshot record: {}", filename);
    
    let db_path = get_database_path(app)?;
    let conn = Connection::open(&db_path)
        .map_err(|e| VibeError::DatabaseError(e))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO screenshots (id, filename, filepath, created_at, filesize, censoring_applied)
         VALUES (?, ?, ?, ?, ?, ?)",
        params![id, filename, filepath, now, filesize, false],
    ).map_err(|e| VibeError::DatabaseError(e))?;
    
    info!("Screenshot record stored with ID: {}", id);
    Ok(id)
}

/// Retrieve screenshot history
pub fn get_screenshot_history(app: &AppHandle, limit: usize) -> Result<Vec<ScreenshotRecord>> {
    info!("Retrieving screenshot history (limit: {})", limit);
    
    let db_path = get_database_path(app)?;
    let conn = Connection::open(&db_path)
        .map_err(|e| VibeError::DatabaseError(e))?;
    
    let mut stmt = conn.prepare(
        "SELECT id, filename, filepath, created_at, filesize, censoring_applied
         FROM screenshots
         ORDER BY created_at DESC
         LIMIT ?"
    ).map_err(|e| VibeError::DatabaseError(e))?;
    
    let records = stmt.query_map([limit as i32], |row| {
        Ok(ScreenshotRecord {
            id: row.get(0)?,
            filename: row.get(1)?,
            filepath: row.get(2)?,
            created_at: row.get(3)?,
            filesize: row.get(4)?,
            censoring_applied: row.get(5)?,
        })
    }).map_err(|e| VibeError::DatabaseError(e))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| VibeError::DatabaseError(e))?;
    
    info!("Retrieved {} screenshot records", records.len());
    Ok(records)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenshotRecord {
    pub id: String,
    pub filename: String,
    pub filepath: String,
    pub created_at: String,
    pub filesize: i64,
    pub censoring_applied: bool,
}
