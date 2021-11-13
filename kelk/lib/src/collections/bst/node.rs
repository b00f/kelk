#[repr(C)]
pub(super) struct Node<K: Sized + Ord, V: Sized> {
    pub left: u32,
    pub right: u32,
    pub key: K,
    pub value: V,
}

impl<K: Sized + Ord, V: Sized> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: 0,
            right: 0,
        }
    }
}
