use crate::context::ContextAPI;
use crate::error::KelkError;
use crate::params::*;
use alloc::vec::Vec;

/// The raw return code returned by the host side.
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ReturnCode {
    /// The result has no error
    Success = 0,
}

/// todo
#[link(wasm_import_module = "zarb")]
extern "C" {
    /// todo
    fn write_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
    /// todo
    fn read_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
}

/// TODO
pub struct ContextExt {}

impl ContextExt {
    /// TODO
    pub fn new() -> Self {
        ContextExt {}
    }
}

/// todo
impl ContextAPI for ContextExt {
    //todo
    fn msg_sender(&self) -> Result<Vec<u8>, KelkError> {
        unimplemented!()
    }
    /// todo
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        let ptr = data.as_ptr() as u32;
        let len = data.len() as u32;

        let read = unsafe { write_storage(offset, ptr, len) };
        if read != ReturnCode::Success {
            return Err(KelkError::WriteStorageFailed);
        }
        Ok(())
    }

    /// todo
    fn read_storage(&self, offset: u32, len: u32) -> Result<Vec<u8>, KelkError> {
        let mut vec = alloc::vec::Vec::with_capacity(len as usize);
        vec.resize(len as usize, 0);
        let ptr = vec.as_ptr() as u32;

        let read = unsafe { read_storage(offset, ptr, len) };
        if read != ReturnCode::Success {
            return Err(KelkError::WriteStorageFailed);
        }
        Ok(vec)
    }

    /// todo
    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!();
    }
}
