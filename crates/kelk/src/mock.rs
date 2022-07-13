//! The context for running contract actor

use crate::{
    blockchain::mock::mock_blockchain, context::OwnedContext, storage::mock::mock_storage,
};

/// mocks the context for testing
pub fn mock_context(storage_size: usize) -> OwnedContext {
    OwnedContext {
        blockchain: mock_blockchain(),
        storage: mock_storage(storage_size),
    }
}
