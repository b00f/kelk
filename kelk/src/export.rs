//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::context::{ContextExt, ContextMut, OwnedContext};
use serde::{de::DeserializeOwned, Serialize};

/// TODO
#[derive(Debug, Serialize)]
pub struct Response {
    // TODO: vec<u8> is a bad practice. look for a better response structure
    // data: Vec<u8>
    /// TODO
    pub res: i32
}

/// do_execute should be wrapped in an external "C" export, containing a contract-specific function as arg
///
/// - `M`: message type for request
/// - `E`: error type for responses
pub fn do_process<M, E>(
    execute_fn: &dyn Fn(ContextMut, M) -> Result<Response, E>,
    msg_ptr: u32,
) -> u32
where
    M: DeserializeOwned,
    E: Serialize,
{
    0
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}
