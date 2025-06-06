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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found() {
        let error = StorageError::NotFound;
        assert!(error.is_not_found());
        assert!(!error.is_already_exists());
    }

    #[test]
    fn test_already_exists() {
        let error = StorageError::AlreadyExists;
        assert!(error.is_already_exists());
        assert!(!error.is_not_found());
    }
}
