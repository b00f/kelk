//! The context for running contract actor

use crate::{blockchain::Blockchain, storage::Storage};

/// `Context` owns the `ContextAPI` reference.
pub struct Context<'a> {
    /// The instance of Blockchain APIs
    pub blockchain: &'a Blockchain,

    /// The instance of storage APIs
    pub storage: &'a Storage,
}

/// `OwnedContext` owns the `ContextAPI` instance. It allow dependency injection at runtime.
/// This cannot be copied or cloned since `api` doesn't implement Copy and Clone traits.
/// It can be easily mocked for the testing environment.
pub struct OwnedContext {
    /// The instance of mocked Blockchain
    pub blockchain: Blockchain,
    /// The instance of mocked Storage
    pub storage: Storage,
}

impl OwnedContext {
    /// returns the context as reference
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context {
            blockchain: &self.blockchain,
            storage: &self.storage,
        }
    }
}
