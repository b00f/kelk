//! Mocking Context for testing contracts

use crate::error::HostError;
use crate::{blockchain::Blockchain, context::OwnedContext, params::ParamType, storage::Storage};
use alloc::vec::Vec;
use core::cell::RefCell;

/// makes a mocked context
pub fn mock_context(storage_size: usize) -> OwnedContext<MockBlockchain, MockStorage> {
    OwnedContext {
        blockchain: MockBlockchain::new(),
        storage: MockStorage::new(storage_size),
    }
}

/// `MockStorage` mocks the storage for testing purpose.
pub struct MockStorage {
    storage: RefCell<Vec<u8>>,
}

impl MockStorage {
    /// instantiates a new storage mock
    pub fn new(size: usize) -> Self {
        let storage = RefCell::new(alloc::vec![0; size].to_vec());
        Self { storage }
    }
}

/// `MockBlockchain` mocks the blockchain for testing purpose.
pub struct MockBlockchain {}

impl MockBlockchain {
    /// instantiates a new blockchain mock
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MockBlockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain for MockBlockchain {
    fn get_param(&self, _param_id: i32) -> Option<ParamType> {
        unimplemented!()
    }
}

impl Storage for MockStorage {
    fn swrite(&self, offset: u32, data: &[u8]) -> Result<(), HostError> {
        if offset as usize + data.len() > self.storage.borrow().len() {
            return Err(HostError { code: 1 });
        }
        for (i, d) in data.iter().enumerate() {
            self.storage.borrow_mut()[i + offset as usize] = *d;
        }
        Ok(())
    }

    fn sread(&self, offset: u32, length: u32) -> Result<Vec<u8>, HostError> {
        if (offset + length) as usize > self.storage.borrow().len() {
            return Err(HostError { code: 1 });
        }
        let c = &self.storage.borrow()[offset as usize..(offset + length) as usize];
        Ok(c.into())
    }
}

/// mocks the storage
pub fn mock_storage(storage_size: usize) -> MockStorage {
    MockStorage::new(storage_size)
}
