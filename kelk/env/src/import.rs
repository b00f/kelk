use crate::alloc::vec::Vec;
use crate::blockchain::Blockchain;
use crate::error::HostError;
use crate::params::*;
use crate::storage::Storage;

#[cfg(not(test))]
#[link(wasm_import_module = "zarb")]
extern "C" {
    /// write data at given offset of storage file.
    /// `ptr` is the location in sandbox memory where data should be read from.
    /// `len` is the length of data.
    fn write_storage(offset: u32, ptr: u32, len: u32) -> i32;
    /// read data from the given offset of storage file .
    /// `ptr` is the location in sandbox memory where data should be written to.
    /// `len` is the length of data.
    fn read_storage(offset: u32, ptr: u32, len: u32) -> i32;
}

pub(crate) struct ContextExt {}

impl ContextExt {
    pub fn new() -> Self {
        ContextExt {}
    }
}

// TODO:
// Is it possible to create a zarb module for testing and remove these code?
#[cfg(test)]
pub unsafe fn write_storage(_offset: u32, _ptr: u32, _len: u32) -> i32 {
    0
}

#[cfg(test)]
pub unsafe fn read_storage(_offset: u32, _ptr: u32, _len: u32) -> i32 {
    0
}

impl Storage for ContextExt {
    fn swrite(&self, offset: u32, data: &[u8]) -> Result<(), HostError> {
        let ptr = data.as_ptr() as u32;
        let len = data.len() as u32;

        let code = unsafe { write_storage(offset, ptr, len) };
        if code != 0 {
            return Err(HostError { code });
        }
        Ok(())
    }

    fn sread(&self, offset: u32, len: u32) -> Result<Vec<u8>, HostError> {
        let vec = crate::alloc::vec![0; len as usize];
        let ptr = vec.as_ptr() as u32;

        let code = unsafe { read_storage(offset, ptr, len) };
        if code != 0 {
            return Err(HostError { code });
        }
        Ok(vec)
    }
}

impl Blockchain for ContextExt {
    /// todo
    fn get_param(&self, _param_id: i32) -> Option<ParamType> {
        unimplemented!();
    }
}
