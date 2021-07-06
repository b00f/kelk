#![no_std]
#![no_main]

pub fn sum() -> i32 {
    return 1 + 2;
}

/// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn deploy() {}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() -> i32 {
    sum()
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