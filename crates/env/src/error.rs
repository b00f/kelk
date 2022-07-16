//! Define the Host error.

/// Error raised by the host
#[derive(Debug)]
pub struct HostError {
    /// The error code
    pub code: i32,
}
