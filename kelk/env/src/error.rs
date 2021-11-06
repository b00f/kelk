//! Kelk errors.

/// Kelk error types
pub enum KelkError {
    /// An issue occurred on writing into storage file
    WriteStorageFailed,
    /// if user try to access out of bound offset
    StorageOutOfBound,
}
