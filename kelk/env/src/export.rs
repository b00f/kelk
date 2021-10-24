//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::context::{ContextMut, OwnedContext};
use crate::import::ContextExt;
use crate::memory;
use crate::Response;
use minicbor::{Decode, Encode};

/// allocate reserves the given number of bytes in wasm memory and returns a pointer
/// to the this data. This space is managed by the calling process and
/// should be accompanied by a corresponding deallocate
#[no_mangle]
extern "C" fn allocate(size: usize) -> u32 {
    memory::allocate(size)
}

/// deallocate frees the allocate memory.
#[no_mangle]
extern "C" fn deallocate(ptr: u32) {}

/// TODO
pub fn do_instantiate<E: Encode>(instantiate_fn: &dyn Fn(ContextMut) -> E) -> u32 {
    let mut ctx = make_context();
    instantiate_fn(ctx.as_mut());
    0
}

/// TODO: UPDATE MY COMMENT
/// do_process_msg should be wrapped in an external "C" export, containing a contract-specific function as arg
///
/// - `M`: message type for request
/// - `E`: error type for responses
pub fn do_process_msg<'a, D: Decode<'a>, E: Encode>(
    process_msg_fn: &dyn Fn(ContextMut, D) -> E,
    msg_ptr: *const u8,
    length: u32,
) -> u64 {
    let buf: &[u8] = unsafe { core::slice::from_raw_parts(msg_ptr, length as usize) };
    let msg = minicbor::decode(buf).unwrap(); // TODO: return error
    let mut ctx = make_context();
    let res = process_msg_fn(ctx.as_mut(), msg);

    result_to_region(res)
}

fn result_to_region<E: Encode>(res: E) -> u64 {
    let mut vec = alloc::vec::Vec::new();
    minicbor::encode(res, &mut vec).unwrap();

    let ptr = vec.as_ptr() as u64;
    let len = vec.len() as u64;

    core::mem::forget(vec);
    ptr | (len << 32)
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}
