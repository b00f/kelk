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

#[link(wasm_import_module = "zarb")]
extern "C" {
    /// write data at given offset of storage file.
    /// `ptr` is the location in sandbox memory where data should be read from.
    /// `len` is the length of data.
    fn write_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
    /// read data from the given offset of storage file .
    /// `ptr` is the location in sandbox memory where data should be written to.
    /// `len` is the length of data.
    fn read_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
}

pub(crate) struct ContextExt {}

impl ContextExt {
    pub fn new() -> Self {
        ContextExt {}
    }
}

impl ContextAPI for ContextExt {
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        let ptr = data.as_ptr() as u32;
        let len = data.len() as u32;

        let read = unsafe { write_storage(offset, ptr, len) };
        if read != ReturnCode::Success {
            return Err(KelkError::WriteStorageFailed);
        }
        Ok(())
    }

    fn read_storage(&self, offset: u32, len: u32) -> Result<Vec<u8>, KelkError> {
        let vec = alloc::vec![0; len as usize];
        let ptr = vec.as_ptr() as u32;

        let read = unsafe { read_storage(offset, ptr, len) };
        if read != ReturnCode::Success {
            return Err(KelkError::WriteStorageFailed);
        }
        Ok(vec)
    }

    /// todo
    fn get_param(&self, _param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!();
    }
}
