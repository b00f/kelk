use core::mem::size_of;

#[repr(C)]
pub(super) struct Header {
    pub boom: u32,
    pub key_len: u16,
    pub value_len: u16,
    pub count: u32,
    pub capacity: u32,
}

impl Header {
    pub fn new<K: Sized, V: Sized>(capacity: u32) -> Self {
        Self {
            boom: 0xb3000000,
            key_len: size_of::<K>() as u16,
            value_len: size_of::<V>() as u16,
            count: 0,
            capacity,
        }
    }
}
