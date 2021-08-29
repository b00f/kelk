use crate::ReturnCode;


/// The raw return code returned by the host side.
#[derive(Debug)]
#[repr(u32)]
pub enum ReturnCode {
    /// The result has no error
    Success = 0,
}


#[link(wasm_import_module = "zarb")]
extern "C" {
    fn write_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
    fn read_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
}
