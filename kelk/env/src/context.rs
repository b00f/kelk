//! The context for running contract actor

use crate::{blockchain::Blockchain, storage::Storage};

/// `Context` owns the `ContextAPI` reference.
pub struct Context<'a> {
    /// The instance of Blockchain APIs
    pub blockchain: &'a dyn crate::blockchain::Blockchain,

    /// The instance of storage APIs
    pub storage: &'a dyn crate::storage::Storage,
}

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
