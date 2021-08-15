#![no_std]
use core::str;


pub fn set_greeting(name: &str) {
    let mut value = name.as_bytes();
    // TODO: make it fixed length
    //value.resize(16, 0);
    kelk::write_storage(0, value);
}

pub fn greeting() {
    let value = &[0 as u8;16];
    kelk::read_storage(0, value);
    let greeting =str::from_utf8(value).unwrap();
    kelk::println(greeting);
}

/// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn deploy() {
    set_greeting("hello world!");
}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() {
    greeting();
}
