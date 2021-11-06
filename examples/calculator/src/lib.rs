#![no_std]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler))]
#![feature(lang_items)]

pub mod contract;
pub mod error;
pub mod message;

// TODO: Move these methods to kelk? Is it possible?

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
