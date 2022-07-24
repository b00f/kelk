use crate::storage::codec::Codec;
use crate::storage::Offset;
use crate::Codec;
use alloc::vec::Vec;


#[derive(Codec)]
pub(super) struct Header {
    pub count: u32,
    pub item_len: u16,
    pub head_offset: Offset,
    pub tail_offset: Offset,
}

impl Header {
    pub fn new<T: Codec>() -> Self {
        Self {
            count: 0,
            item_len: T::PACKED_LEN as u16,
            head_offset: 0,
            tail_offset: 0,
        }
    }
}
