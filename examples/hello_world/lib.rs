#![no_std]



#[no_mangle]
pub fn greeting() {
    kelk::println("hello world");
}


/// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn deploy() {
}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() {
}