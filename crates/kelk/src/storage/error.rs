//! Storage error types

use alloc::string::String;
use core::fmt::{self, Debug};

/// A general list of Storage Binary Tree error
#[derive(Debug)]
pub enum Error {
    /// Host error code
    HostError(i32),

    /// Invalid offset
    InvalidOffset(u32),

    /// Capacity is full
    OutOfCapacity,

    /// Generic error
    GenericError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HostError(code) => write!(f, "host error code: {}", code),
            Error::InvalidOffset(offset) => write!(f, "invalid offset: {}", offset),
            Error::OutOfCapacity => write!(f, "Capacity is full"),
            Error::GenericError(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl From<kelk_env::error::HostError> for Error {
    fn from(error: kelk_env::error::HostError) -> Self {
        Error::HostError(error.code)
    }
}
