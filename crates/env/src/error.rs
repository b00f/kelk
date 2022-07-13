//! Define the Host error.

use alloc::fmt;
use core::fmt::Display;

/// Error raised by the host
#[derive(Debug)]
pub struct HostError {
    /// The error code
    pub code: i32,
}

// impl Debug for HostError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("HostError")
//             .field("code", &self.code)
//             .finish()
//     }
// }

impl Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "host error code: {:?}", self.code)
    }
}
