use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Not Found")]
    NotFound,
    #[error("Already Exists")]
    AlreadyExists,
}

impl StorageError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, StorageError::NotFound)
    }

    pub fn is_already_exists(&self) -> bool {
        matches!(self, StorageError::AlreadyExists)
    }
}
