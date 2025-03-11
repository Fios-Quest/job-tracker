mod company;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StorageError {
    NotFound,
    AlreadyExists,
}
