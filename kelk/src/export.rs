//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::context::{ContextExt, ContextMut, OwnedContext};
use crate::Response;
use minicbor::{Decode, Encode};

/// TODO
pub fn do_instantiate<E>(instantiate_fn: &dyn Fn(ContextMut) -> Result<Response, E>) -> u32
where
    E: Encode,
{
    let mut ctx = make_context();
    instantiate_fn(ctx.as_mut());
    0
}

/// do_execute should be wrapped in an external "C" export, containing a contract-specific function as arg
///
/// - `M`: message type for request
/// - `E`: error type for responses
pub fn do_process_msg<'a, D, E>(
    process_msg_fn: &dyn Fn(ContextMut, D) -> Result<Response, E>,
    msg_ptr: *const u8,
    length: u32,
) -> u32
where
    D: Decode<'a>,
    E: Encode,
{
    let buf: &[u8] = unsafe { core::slice::from_raw_parts(msg_ptr, length as usize) };
    let msg = minicbor::decode(buf).unwrap(); // TODO: return error
    let mut ctx = make_context();
    process_msg_fn(ctx.as_mut(), msg);
    0 // TODO: convert res to ptr??
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}
