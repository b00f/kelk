//! Modules definition for storage libraries

pub mod collections;
pub mod error;
#[cfg(test)]
pub mod mock;
pub mod storage;

pub use storage::Storage;
