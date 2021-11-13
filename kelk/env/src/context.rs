//! The context for running contract actor

use crate::params::*;

/// `ContextAPI` provides the necessary APIs to interact with the Tanour.
/// It can't be copied or cloned since it doesn't have Copy and Clone traits.
pub trait ContextAPI: kelk_lib::storage::Storage {
    /// TODO move it to lib crate
    /// gets the parameter value
    fn get_param(&self, param_id: i32) -> Option<ParamType>;
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
