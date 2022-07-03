//! Mocking Context for testing contracts

use crate::error::HostError;
use crate::{blockchain::Blockchain, context::Context, params::ParamType, storage::Storage};
use alloc::vec::Vec;
use core::cell::RefCell;

/// `OwnedContext` owns the `ContextAPI` instance. It allow dependency injection at runtime.
/// This cannot be copied or cloned since `api` doesn't implement Copy and Clone traits.
/// It can be easily mocked for the testing environment.
pub struct OwnedContext<B: Blockchain, S: Storage> {
    /// The instance of mocked Blockchain
    pub blockchain: B,
    /// The instance of mocked Storage
    pub storage: S,
}

impl<B: Blockchain, S: Storage> OwnedContext<B, S> {
    /// returns the context as reference
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context {
            blockchain: &self.blockchain,
            storage: &self.storage,
        }
    }
}

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
