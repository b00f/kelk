//! Mocking the storage for testing purpose

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::{cell::RefCell, result::Result};
use kelk_env::{Error, StorageAPI};

use super::Storage;

/// mocks the storage for testing purpose.
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

impl StorageAPI for MockStorage {
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

/// mocks the storage for testing
pub fn mock_storage(storage_size: usize) -> Storage {
    let storage = MockStorage::new(storage_size);
    Storage {
        api: Box::new(storage),
    }
}
