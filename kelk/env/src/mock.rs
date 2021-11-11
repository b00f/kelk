//! Mocking Context for testing contracts

use crate::{
    context::{ContextAPI, OwnedContext},
    error::KelkError,
    params::ParamType,
};
use kelk_lib;
use kelk_lib::alloc::vec::Vec;
use kelk_lib::mock::MockStorage;
use kelk_lib::storage::{Error, Storage};

/// `MockContextAPI` mocks the APIs for testing purpose.
pub struct MockContextAPI {
    storage_mock: MockStorage,
}

impl MockContextAPI {
    /// instantiates a new mock
    pub fn new(size: usize) -> Self {
        MockContextAPI {
            storage_mock: MockStorage::new(size),
        }
    }
}

impl Storage for MockContextAPI {
    fn swrite(&self, offset: u32, data: &[u8]) -> Result<(), Error> {
        self.storage_mock.swrite(offset, data)
    }

    fn sread(&self, offset: u32, len: u32) -> Result<Vec<u8>, Error> {
        self.storage_mock.sread(offset, len)
    }
}

impl ContextAPI for MockContextAPI {
    fn get_param(&self, _param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!()
    }
}

/// makes a mocked context
pub fn mock_context(storage_size: usize) -> OwnedContext<MockContextAPI> {
    let api = MockContextAPI::new(storage_size);
    OwnedContext { api }
}
