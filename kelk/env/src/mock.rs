//! Mocking Context for testing contracts

use crate::error::Error;
use crate::storage::Storage;
use crate::{
    blockchain::Blockchain, context::OwnedContext, params::ParamType, storage::StorageAPI,
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;

/// mocks the context for testing
pub fn mock_context(storage_size: usize) -> OwnedContext<MockBlockchain> {
    OwnedContext {
        blockchain: MockBlockchain::new(),
        storage: mock_storage(storage_size),
    }
}

/// mocks the storage for testing
pub fn mock_storage(storage_size: usize) -> Storage {
    let storage = MockStorageAPI::new(storage_size);
    Storage {
        api: Box::new(storage),
    }
}

/// mocks the storage for testing purpose.
pub struct MockStorageAPI {
    storage: RefCell<Vec<u8>>,
}

impl MockStorageAPI {
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

impl StorageAPI for MockStorageAPI {
    fn write(&self, offset: u32, data: &[u8]) -> Result<(), Error> {
        if offset as usize + data.len() > self.storage.borrow().len() {
            return Err(Error::GenericError("overflowed"));
        }
        for (i, d) in data.iter().enumerate() {
            self.storage.borrow_mut()[i + offset as usize] = *d;
        }
        Ok(())
    }

    fn read(&self, offset: u32, length: u32) -> Result<Vec<u8>, Error> {
        if (offset + length) as usize > self.storage.borrow().len() {
            return Err(Error::GenericError("overflowed"));
        }
        let c = &self.storage.borrow()[offset as usize..(offset + length) as usize];
        Ok(c.into())
    }
}
