//! The context for running contract actor

use crate::error::KelkError;
use alloc::vec::Vec;
use minicbor::{Decode, Encode};

///
pub const PARAM_CALLER_ADDRESS: i32 = 0x0010;
///
pub const PARAM_CALLER_ID: i32 = 0x0011;


/// Parameter value types
#[derive(Encode, Decode)]
pub enum ParamType {
    /// A 32-bit integer.
    #[n(0)]
    I32 {
        #[doc(hidden)]
        #[n(0)]
        value: i32,
    },
    /// A 64-bit integer.
    #[n(1)]
    I64 {
        #[doc(hidden)]
        #[n(0)]
        value: i64,
    },
    // #[n(2)]
    // I128 {
    //     #[n(0)]
    //     value: i128,
    // },
    // #[n(10)]
    // Address {
    //     #[n(0)]
    //     value: [u8; 20],
    // },
}

/// TODO
pub trait ContextAPI {
    /// TODO
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError>;

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

    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError> {
        todo!("unimplemented");
    }

}
