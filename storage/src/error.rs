use serde::de;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Not Found")]
    NotFound,
    #[error("Already Exists")]
    AlreadyExists,
    #[error("Deserialisation error: {0}")]
    DeserializationError(de::value::Error),
    #[error("File IO error: {0}")]
    TokioIoError(#[from] tokio::io::Error),
    #[error("Json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl StorageError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, StorageError::NotFound)
    }

    pub fn is_already_exists(&self) -> bool {
        matches!(self, StorageError::AlreadyExists)
    }
}
