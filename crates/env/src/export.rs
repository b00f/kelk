//! Exported WASM functions
//!
//! `do_instantiate`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[kelk_derive(...)]` macro attribute.

use crate::memory::Pointer;

/// allocate reserves the given number of bytes in wasm memory and returns a pointer
/// to a Pointer defining this data. This space is managed by the calling process
/// and should be accompanied by a corresponding deallocate
#[no_mangle]
extern "C" fn allocate(size: u32) -> u64 {
    Pointer::allocate(size).as_u64()
}

/// deallocate expects a pointer to a Pointer created with allocate.
/// It will free both the Pointer and the memory referenced by the Pointer.
#[no_mangle]
extern "C" fn deallocate(ptr_u64: u64) {
    Pointer::from_u64(ptr_u64).deallocate();
}
