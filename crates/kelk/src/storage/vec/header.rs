use crate::storage::codec::Codec;
use crate::{storage::Offset, Codec};
use alloc::vec::Vec;

#[derive(Codec)]
pub(super) struct Header {
    pub count: u32,
    pub capacity: u32,
    pub value_len: u16,
    pub data_offset: Offset,
}

impl Header {
    pub fn new<T: Codec>(capacity: u32, data_offset: Offset) -> Self {
        Self {
            value_len: T::PACKED_LEN as u16,
            count: 0,
            capacity,
            data_offset,
        }
    }
}
