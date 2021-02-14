#![no_std]


const STORAGE_KEY : u32 = 1;


#[no_mangle]
pub fn set(val: u32) {
    kelk::set_storage(&STORAGE_KEY, &val.into());
}

#[no_mangle]
pub fn get() -> u32 {

    kelk::get_storage(&STORAGE_KEY)
}
