#![no_std]

/// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // SAFETY: We only use this operation if we are guaranteed to be in Wasm32 compilation.
    //         This is used in order to make any panic a direct abort avoiding Rust's general
    //         panic infrastructure.
    unsafe {
        core::arch::wasm32::unreachable();
    }
}