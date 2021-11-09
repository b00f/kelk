//! Kelk errors.

/// Kelk error types
pub enum KelkError {
    /// The allocated memory is invalid
    InvalidMemory,
    /// An issue occurred on writing into storage file
    WriteStorageFailed,
    /// if user try to access out of bound offset
    StorageOutOfBound,
}
