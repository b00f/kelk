use core::mem;
use alloc::vec::Vec;

/// Allocates the memory of capacity `size` and length 0. Returns a pointer.
/// This is the same as the `allocate` export, but designed to be called internally.
pub fn allocate(size: usize) -> u32 {
    let data: Vec<u8> = Vec::with_capacity(size);
    let data_ptr = data.as_ptr() as u32;

    mem::forget(data);
    data_ptr
}
