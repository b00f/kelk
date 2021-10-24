//! Kelk-env is a Low-level interface for interacting with Tanour (Wasm executor) in Zarb blockchain.
//!
#![no_std]
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

pub mod context;
pub mod error;
pub mod mock;
pub mod params;

#[cfg(target_arch = "wasm32")]
mod memory;

#[cfg(target_arch = "wasm32")]
mod import;

#[cfg(target_arch = "wasm32")]
pub mod export;

#[cfg(target_arch = "wasm32")]
pub use crate::export::{do_instantiate, do_process_msg, do_query};

pub use kelk_derive::kelk_derive;
