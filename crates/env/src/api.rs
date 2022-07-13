//! Defining the Kelk API trait.

use crate::error::HostError;
use alloc::vec::Vec;

/// the storage APIs that should be provided by the host.
/// It can't be copied or cloned since it doesn't have Copy and Clone traits.
pub trait StorageAPI {
    /// This API requests the host to read data from the storage file
    /// at the given `offset` up to the given `length`.
    fn read(&self, offset: u32, length: u32) -> Result<Vec<u8>, HostError>;

    /// This API requests the host to write `data` into the storage file
    /// at the given `offset`
    fn write(&self, offset: u32, data: &[u8]) -> Result<(), HostError>;
}

/// the blockchain APIs that should be provided by the host.
/// It can't be copied or cloned since it doesn't have Copy and Clone traits.
pub trait BlockchainAPI {
    /// This API requests the host to return the associated value to the given
    /// `param_id`.
    fn get_param(&self, param_id: u32) -> Result<Vec<u8>, HostError>;
}
