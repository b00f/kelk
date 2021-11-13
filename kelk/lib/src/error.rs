//! Define the Host error.

use core::fmt::{self, Debug};

/// Error raised by the host
pub struct HostError {
    /// Error raised by the host
    pub code: i32,
}

impl Debug for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostError")
            .field("code", &self.code)
            .finish()
    }
}
impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Host error. Code: {:?}", self.code)
    }
}
