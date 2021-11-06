//! Mocking Context for testing contracts

use alloc::vec::Vec;
use core::cell::RefCell;

use crate::{
    context::{ContextAPI, OwnedContext},
    error::KelkError,
    params::ParamType,
};

/// `MockContextAPI` mocks the APIs for testing purpose.
pub struct MockContextAPI {
    storage: RefCell<Vec<u8>>,
}

impl MockContextAPI {
    /// instantiates a new mock
    pub fn new(size: u32) -> Self {
        let storage = RefCell::new(Vec::with_capacity(size as usize));
        storage.borrow_mut().fill(0);
        MockContextAPI { storage }
    }
}

impl ContextAPI for MockContextAPI {
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        if offset as usize + data.len() > self.storage.borrow().len() {
            return Err(KelkError::StorageOutOfBound);
        }
        for (i, d) in data.iter().enumerate() {
            self.storage.borrow_mut()[i + offset as usize] = *d;
        }
        Ok(())
    }

    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError> {
        let c = &self.storage.borrow()[offset as usize..offset as usize + length as usize];
        Ok(c.into())
    }

    fn get_param(&self, _param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!()
    }
}

/// makes a mocked context
pub fn mock_context(storage_size: u32) -> OwnedContext<MockContextAPI> {
    let api = MockContextAPI::new(storage_size);
    OwnedContext { api }
}
