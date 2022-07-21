use core::mem::size_of;

#[repr(C)]
pub(super) struct Header {
    pub reserved: u16,
    pub value_len: u16,
    pub count: u32,
    pub capacity: u32,
}

impl Header {
    pub fn new<V: Sized>(capacity: u32) -> Self {
        Self {
            reserved: 0,
            value_len: size_of::<V>() as u16,
            count: 0,
            capacity,
        }
    }
}
