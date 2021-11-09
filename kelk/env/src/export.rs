//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::context::{Context, OwnedContext};
use crate::import::ContextExt;
use crate::memory::Pointer;
use minicbor::{Decode, Encode};

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

/// TODO
pub fn do_instantiate<E: Encode>(instantiate_fn: &dyn Fn(Context) -> E) -> u32 {
    let ctx = make_context();
    instantiate_fn(ctx.as_ref());
    0
}

/// TODO: UPDATE MY COMMENT
/// do_process_msg should be wrapped in an external "C" export, containing a contract-specific function as arg
///
/// - `M`: message type for request
/// - `E`: error type for responses
pub fn do_process_msg<'a, D: Decode<'a>, E: Encode>(
    process_msg_fn: &dyn Fn(Context, D) -> E,
    msg_ptr: u64,
) -> u64 {
    let ptr = Pointer::from_u64(msg_ptr);
    let buf = unsafe { ptr.to_slice() };
    let msg = minicbor::decode(buf).expect("Decoding failed");
    let ctx = make_context();
    let res = process_msg_fn(ctx.as_ref(), msg);

    result_to_ptr(res)
}

fn result_to_ptr<E: Encode>(res: E) -> u64 {
    let mut vec = alloc::vec::Vec::new();
    minicbor::encode(res, &mut vec).expect("Encoding failed");

    Pointer::release_buffer(vec).as_u64()
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}

#[cfg(test)]
mod tests {
    // Uncomment this test if should_panic supported by wasm_bindgen_test.
    // https://github.com/rustwasm/wasm-bindgen/issues/2286
    //
    // use super::*;
    // use wasm_bindgen_test::*;
    //
    // #[wasm_bindgen_test]
    // #[should_panic]
    // fn test_allocation() {
    //     let ptr = allocate(1);
    //     deallocate(ptr);

    //     // Should panic here, because the pointer is freed before
    //     deallocate(ptr);
    // }
}
