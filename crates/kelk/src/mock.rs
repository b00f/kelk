//! The context for running contract actor

use crate::{
    blockchain::{mock::MockBlockchain, Blockchain},
    context::Context,
    storage::{mock::MockStorage, Storage},
};

/// `MockedContext` owns the mocked instances.
pub struct MockedContext {
    /// The instance of Storage
    pub storage: Storage,
    /// The instance of Blockchain
    pub blockchain: Blockchain,
}

impl MockedContext {
    /// returns the context as reference
    pub fn as_ref(&self) -> Context<'_> {
        Context {
            storage: &self.storage,
            blockchain: &self.blockchain,
        }
    }

    /// returns a reference to the mocked storage
    pub fn mocked_storage(&self) -> &MockStorage {
        self.storage
            .api
            .as_any()
            .downcast_ref::<MockStorage>()
            .expect("Wasn't a trusty printer!")
    }

    /// returns a reference to the mocked blockchain
    pub fn mocked_blockchain(&self) -> &MockBlockchain {
        self.blockchain
            .api
            .as_any()
            .downcast_ref::<MockBlockchain>()
            .expect("Wasn't a trusty printer!")
    }
}

/// mocks the context for testing
pub fn mock_context(storage_size: usize) -> MockedContext {
    use crate::{blockchain::mock::mock_blockchain, storage::mock::mock_storage};

    MockedContext {
        blockchain: mock_blockchain(),
        storage: mock_storage(storage_size),
    }
}
