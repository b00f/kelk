use core::mem::size_of;

#[repr(C)]
pub(super) struct Header {
    pub boom: u32,
    pub item_len: u16,
    pub count: u32,
    pub capacity: u32,
    pub head_offset: u32,
    pub tail_offset: u32,
}

impl Header {
    pub fn new<I: Sized>(capacity: u32) -> Self {
        Self {
            boom: 0xc3000000,
            item_len: size_of::<I>() as u16,
            count: 0,
            head_offset: 0,
            tail_offset: 0,
            capacity,
        }
    }
}
