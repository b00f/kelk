#![no_std]
#![cfg_attr(not(test), feature(core_intrinsics, lang_items, alloc_error_handler))]

pub mod contract;
pub mod error;

// Use `wee_alloc` as the global allocator.
#[cfg(not(test))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Need to provide a tiny `panic` implementation for `#![no_std]`.
/// This translates into an `unreachable` instruction that will
/// raise a `trap` the WebAssembly execution if we panic at runtime.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort();
}

/// Need to provide an allocation error handler which just aborts
/// the execution with trap.
#[cfg(not(test))]
#[alloc_error_handler]
#[no_mangle]
fn oom(_: core::alloc::Layout) -> ! {
    ::core::intrinsics::abort();
}

/// Needed for non-wasm targets.
#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
