#![no_std]

use kelk::Kelk;

const STORAGE_KEY : u32 = 1;


#[no_mangle]
pub fn set(val: u32) {
    Kelk::new().set_storage::<u32>(&STORAGE_KEY, &val);
}

#[no_mangle]
pub fn get() -> u32 {
    match Kelk::new().get_storage::<u32>(&STORAGE_KEY) {
        Some(val)=> val,
        None => 0
    }
}
