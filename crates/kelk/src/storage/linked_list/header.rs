#[repr(C)]
pub(super) struct Header {
    pub count: u32,
    pub head_offset: u32,
    pub tail_offset: u32,
}

impl Header {
    pub fn new() -> Self {
        Self {
            count: 0,
            head_offset: 0,
            tail_offset: 0,
        }
    }
}
