//! The context for running contract actor

use crate::{blockchain::Blockchain, storage::Storage};

/// `Context` holds the references to the storage and blockchain objects
/// It can be easily mocked for the testing environment.
pub struct Context<'a> {
    /// A reference to the instance Storage
    pub storage: &'a Storage,
    /// A reference to the instance Blockchain
    pub blockchain: &'a Blockchain,
}

/// `OwnedContext` owns the instances.
/// This cannot be copied or cloned since it doesn't implement Copy and Clone traits.
pub struct OwnedContext {
    /// The instance of Storage
    pub storage: Storage,
    /// The instance of Blockchain
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
        storage: mock_storage(storage_size),
    }
}
