#[repr(C)]
pub(super) struct Node<I: Sized> {
    pub item: I,
    pub next: u32,
}

impl<I: Sized> Node<I> {
    pub fn new(item: I) -> Self {
        Self { item, next: 0 }
    }
}
