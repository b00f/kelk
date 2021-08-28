#![no_std]
#![no_main]

pub fn sum(a: i32, b: i32) -> i32 {
    return a+b;
}

pub fn sub(a: i32, b: i32) -> i32 {
    return a-b;
}

pub fn mul(a: i32, b: i32) -> i32 {
    return a*b;
}

pub fn div(a: i32, b: i32) -> i32 {
    return a/b;
}

/// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn deploy() {}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call(func: &str, a: i32, b: i32) -> i32 {
    match func {
        "sum" => sum(a,b),
        "sub" => sub(a,b),
        "mul" => mul(a,b),
        "div" => div(a,b),
        _ => 0
    }
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