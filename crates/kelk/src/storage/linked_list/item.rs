#[repr(C)]
pub(super) struct Item<I: Sized> {
    pub item: I,
    pub next: u32,
}

impl<I: Sized> Item<I> {
    pub fn new(item: I) -> Self {
        Self { item, next: 0 }
    }
}
