use core::mem::size_of;

#[repr(C)]
pub(super) struct Header {
    pub boom: u32,
    pub key_size: u16,
    pub value_size: u16,
    pub count: u32,
    pub reserved: u32,
}

impl Header {
    pub fn new<K: Sized, V: Sized>() -> Self {
        Self {
            boom: 0xb3000000,
            key_size: size_of::<K>() as u16,
            value_size: size_of::<V>() as u16,
            count: 0,
            reserved: 0,
        }
    }
}
