//! The context for running contract actor

#![feature(alloc)]
extern crate alloc;

use crate::error::KelkError;
use alloc::vec::Vec;

/// TODO
pub trait ContextAPI {
    /// TODO
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError>;

    /// TODO
    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError>;
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

/// TODO
pub struct ContextExt {}

impl ContextExt {
    /// TODO
    pub fn new() -> Self {
        ContextExt {}
    }
}

impl ContextAPI for ContextExt {
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        todo!("unimplemented");
    }

    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError> {
        todo!("unimplemented");
    }
}
