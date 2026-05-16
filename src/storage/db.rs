use crate::app_state::AppState;
use std::io;

#[derive(Debug)]
pub enum StorageError {
    DatabaseError(String),
    IoError(io::Error),
    Custom(String),
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::IoError(err)
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            StorageError::IoError(err) => write!(f, "IO error: {}", err),
            StorageError::Custom(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

pub trait StorageProvider {
    fn load_all(&self) -> Result<AppState, StorageError>;
    fn save_all(&self, state: &AppState) -> Result<(), StorageError>;
    fn init(&self) -> Result<(), StorageError>;
}
