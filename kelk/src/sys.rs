use crate::ReturnCode;

use super::Key;

#[link(wasm_import_module = "zarb")]
extern "C" {
    #[no_mangle]
    pub fn println(ptr: u32, len: u32);

    #[no_mangle]
    pub fn return_value(ptr: u32, len: u32);

    #[no_mangle]
    pub fn write_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;

    #[no_mangle]
    pub fn read_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
}
