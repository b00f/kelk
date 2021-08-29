//! This crate provide APIs for interacting with Zarb blockchain.
//!

#![deny(
    missing_docs,
    bad_style,
    bare_trait_objects,
    const_err,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates
)]

mod kelk;
mod context;
mod sys;
mod memory;

pub use kelk::*;

#[cfg(target_arch = "wasm32")]
mod exports;
#[cfg(target_arch = "wasm32")]
mod imports;

#[cfg(target_arch = "wasm32")]
pub use crate::exports::{do_execute};


/// The raw return code returned by the host side.
#[repr(u32)]
pub enum ReturnCode {
    /// The result has no error
    Success = 0,
    /// The storage key is not found.
    KeyNotFound = 1,
}
