//!
//!

use core::fmt::{Debug, Display};

/// TODO
pub trait Error: Debug + Display {}

/// TODO
pub enum KelkError {
    /// todo
    WriteStorageFailed,
}