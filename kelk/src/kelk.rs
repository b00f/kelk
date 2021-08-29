//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::{
    context::{ContextExt, ContextMut, OwnedContext},
    memory::{consume_region, release_buffer, Region},
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::result::Result;

/// TODO
#[derive(Debug, Serialize)]
pub struct Response {}

/// do_execute should be wrapped in an external "C" export, containing a contract-specific function as arg
///
/// - `M`: message type for request
/// - `E`: error type for responses
pub fn do_process<M, E>(
    execute_fn: &dyn Fn(ContextMut, M) -> Result<Response, E>,
    msg_ptr: u32,
) -> u32
where
    M: DeserializeOwned + JsonSchema,
    E: ToString,
{
    let msg: Vec<u8> = unsafe { consume_region(msg_ptr as *mut Region) };

    let msg: M = serde_json::from_slice(&msg).unwrap(); // TODO: error handling

    let mut context = make_context();
    match execute_fn(context.as_mut(), msg) {
        Ok(res) => {
            let v = serde_json::to_vec(&res).unwrap();
            release_buffer(v) as u32
        }
        Err(_e) => {
            0
        }
    }
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}
