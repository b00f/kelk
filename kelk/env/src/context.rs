//! The context for running contract actor

use crate::error::KelkError;
use crate::params::*;
use alloc::vec::Vec;

/// TODO
pub trait ContextAPI {
    /// TODO
    fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<(), KelkError>;

    /// TODO
    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError>;

    /// TODO
    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError>;
}

/// TODO
pub struct OwnedContext<C: ContextAPI> {
    /// TODO
    pub api: C,
}

/// TODO
pub struct ContextMut<'a> {
    /// TODO
    pub api: &'a dyn ContextAPI,
}

/// TODO
#[derive(Copy, Clone)]
pub struct Context<'a> {
    /// TODO
    pub api: &'a dyn ContextAPI,
}

/// TODO
impl<C: ContextAPI> OwnedContext<C> {
    /// TODO
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context { api: &self.api }
    }

    /// TODO
    pub fn as_mut(&'_ mut self) -> ContextMut<'_> {
        ContextMut { api: &self.api }
    }
}
