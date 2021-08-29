use std::vec::Vec;

use crate::ReturnCode;

pub trait ContextAPI {
    fn write_storage(&self, offset: u32, data: Vec<u8>) -> ReturnCode;
    fn read_storage(&self, offset: u32, length: u32) -> ReturnCode;
}

pub struct OwnedContext<C: ContextAPI> {
    pub api: C,
}

pub struct ContextMut<'a> {
    pub api: &'a dyn ContextAPI,
}

#[derive(Copy, Clone)]
pub struct Context<'a> {
    pub api: &'a dyn ContextAPI,
}

impl<C: ContextAPI> OwnedContext<C> {
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context {
            api: &self.api,
        }
    }

    pub fn as_mut(&'_ mut self) -> ContextMut<'_> {
        ContextMut {
            api: &self.api,
        }
    }
}

pub struct ContextExt {}

impl ContextExt {
    pub fn new() -> Self {
        ContextExt {}
    }
}

impl ContextAPI for ContextExt {
    fn write_storage(&self, offset: u32, data: Vec<u8>) -> ReturnCode {
        todo!("unimplemented");
    }

    fn read_storage(&self, offset: u32, length: u32) -> ReturnCode {
        todo!("unimplemented");
    }
}
