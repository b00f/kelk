//! The context for running contract actor

use crate::error::KelkError;
use crate::params::*;
use alloc::vec::Vec;

/// `ContextAPI` provides the necessary APIs to interact with the Tanour.
/// It can't be copied or cloned since it doesn't have Copy and Clone traits.
pub trait ContextAPI {
    /// Writes `data` into the storage file at the given offset
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError>;

    /// Writes `data` from the storage file at the given offset and length
    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError>;

    /// Gets parameters
    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError>;
}

/// `OwnedContext` owns the `ContextAPI` instance. It allow dependency injection at runtime.
/// This cannot be copied or cloned since `api` doesn't implement Copy and Clone traits.
/// It can be easily mocked for the testing environment.
pub struct OwnedContext<C: ContextAPI> {
    /// The instance of ContextAPI
    pub api: C,
}

/// `Context` owns the `ContextAPI` reference.
pub struct Context<'a> {
    /// The instance of ContextAPI
    pub api: &'a dyn ContextAPI,
}

impl<C: ContextAPI> OwnedContext<C> {
    /// returns the context as reference
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context { api: &self.api }
    }
}
