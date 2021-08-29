use crate::ReturnCode;


#[link(wasm_import_module = "zarb")]
extern "C" {
    fn write_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
    fn read_storage(offset: u32, ptr: u32, len: u32) -> ReturnCode;
}
