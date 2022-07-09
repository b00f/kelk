//! Error types for Storage Binary Tree

use core::fmt::{self, Debug};

/// A general list of Storage Binary Tree error
pub enum Error {
    /// Kelk error
    KelkError,

    /// Invalid offset
    InvalidOffset(u32),

    /// Capacity is full
    OutOfCapacity,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KelkError => f.debug_struct("KelkError").finish(),
            Error::InvalidOffset(offset) => f
                .debug_struct("InvalidOffset")
                .field("offset", &offset)
                .finish(),

            Error::OutOfCapacity => f.debug_struct("Capacity is full").finish(),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KelkError => write!(f, "host error"),
            Error::InvalidOffset(offset) => write!(f, "invalid offset: {:?}", offset),
            Error::OutOfCapacity => write!(f, "Capacity is full"),
        }
    }
}

impl From<kelk_env::error::Error> for Error {
    fn from(_error: kelk_env::error::Error) -> Self {
        Error::KelkError
    }
}
