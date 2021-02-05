use super::error::ReturnCode;
use super::ptr::{Ptr32, Ptr32Mut};
use super::Key;


#[link(wasm_import_module = "zarb")]
extern "C" {
    #[no_mangle]
    pub fn zarb_println(str_ptr: Ptr32<[u8]>, str_len: u32);

    #[no_mangle]
    pub fn zarb_set_storage(key: &Key, value_ptr: Ptr32<[u8]>, value_len: u32);

    #[no_mangle]
    pub fn zarb_get_storage(
        key: &Key,
        output_ptr: Ptr32Mut<[u8]>,
        output_len_ptr: Ptr32Mut<u32>,
    ) -> ReturnCode;
}
