//! The public raw interface towards the host Wasm engine.


use crate::ReturnCode;

use super::sys;
use super::Key;

/// Prints the given contents to the environmental log.
pub fn println(content: &str) {
    let bytes = content.as_bytes();
    unsafe { sys::println(bytes.as_ptr() as u32, bytes.len() as u32) }
}

/// Returns the value back to the caller of the executed contract.
///
/// # Note
///
/// This function  stops the execution of the contract immediately.
pub fn return_value(value: &[u8]) {
    unsafe { sys::return_value(value.as_ptr() as u32, value.len() as u32) }
}

/// Set the value to the contract storage under the given key.
///
pub fn write_storage(offset: u32, value: &[u8]) -> ReturnCode {
    let ret = unsafe {
        sys::write_storage(offset, value.as_ptr() as u32, value.len() as u32)
    };
    ret
}

/// Returns the value stored under the given key in the contract's storage if any.
///
pub fn read_storage(offset: u32, value: &[u8]) -> ReturnCode {
    let ret = unsafe {
        sys::read_storage(offset, value.as_ptr() as u32, value.len() as u32)
    };
    ret
}
