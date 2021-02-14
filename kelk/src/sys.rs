use crate::ReturnCode;

use super::Key;

#[link(wasm_import_module = "zarb")]
extern "C" {
    #[no_mangle]
    pub fn zarb_println(ptr: u32, len: u32);

    #[no_mangle]
    pub fn zarb_return_value(ptr: u32, len: u32);

    #[no_mangle]
    pub fn zarb_set_storage(key: &Key, ptr: u32, len: u32) -> ReturnCode;

    #[no_mangle]
    pub fn zarb_get_storage(key: &Key, ptr: u32) -> ReturnCode;
}
