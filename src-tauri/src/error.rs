use thiserror::Error;

#[derive(Error, Debug)]
pub enum VibeError {
    #[error("Hotkey error: {0}")]
    HotkeyError(String),

    #[error("Screenshot error: {0}")]
    ScreenshotError(String),

    #[error("AI processing error: {0}")]
    AiProcessingError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, VibeError>;

impl serde::Serialize for VibeError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
