//! Define the Host error.

use core::fmt::{self, Debug};

///
pub enum Error {
    /// Error raised by the host
    HostError(i32),
    /// Generic error
    GenericError(&'static str),
}

/// Error raised by the host
pub struct HostError {
    /// Error raised by the host
    pub code: i32,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HostError(code) => f.debug_struct("HostError").field("code", code).finish(),
            Error::GenericError(msg) => f.debug_struct("GenericError").field("msg", msg).finish(),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HostError(code) => write!(f, "host error code: {:?}", code),
            Error::GenericError(msg) => write!(f, "generic code: {:?}", msg),
        }
    }
}
