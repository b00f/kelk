use crate::storage::codec::Codec;
use crate::storage::Offset;
use crate::Codec;
use alloc::vec::Vec;


#[derive(Codec)]
pub(super) struct Node<K: Codec + Ord, V: Codec> {
    pub left: Offset,
    pub right: Offset,
    pub key: K,
    pub value: V,
}

impl<K: Codec + Ord, V: Codec> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: 0,
            right: 0,
        }
    }
}
