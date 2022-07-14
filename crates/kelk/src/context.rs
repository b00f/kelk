//! The context for running contract actor

use crate::{blockchain::Blockchain, storage::Storage};
use alloc::boxed::Box;

/// `Context` owns the `ContextAPI` reference.
pub struct Context<'a> {
    /// The instance of storage APIs
    pub storage: &'a Storage,
    /// The instance of Blockchain APIs
    pub blockchain: &'a Blockchain,
}

/// `OwnedContext` owns the `ContextAPI` instance. It allow dependency injection at runtime.
/// This cannot be copied or cloned since `api` doesn't implement Copy and Clone traits.
/// It can be easily mocked for the testing environment.
pub struct OwnedContext{
    /// The instance of mocked Storage
    pub storage: Box<Storage>,
    /// The instance of mocked Blockchain
    pub blockchain: Blockchain,
}

impl OwnedContext {
    /// returns the context as reference
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context {
            storage: &self.storage,
            blockchain: &self.blockchain,
        }
    }
}
/// mocks the context for testing
pub fn mock_context(storage_size: usize) -> OwnedContext {
    use crate::{blockchain::mock::mock_blockchain, storage::mock::mock_storage};

    OwnedContext {
        blockchain: mock_blockchain(),
        storage: Box::new(mock_storage(storage_size)),
    }
}
