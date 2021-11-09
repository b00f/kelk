use alloc::vec::Vec;

/// defines a pointer to the allocated space in Wasm's linear memory.
pub struct Pointer {
    /// The pointer to the allocated memory
    pub ptr: *const u8,
    /// The length of allocated memory
    pub len: u32,
}

impl Pointer {
    /// defiles a pointer from u64
    pub fn from_u64(ptr_64: u64) -> Self {
        let ptr = (ptr_64 & 0xFFFFFFFF) as *const u8;
        let len = (ptr_64 >> 32) as u32;

        Self { ptr, len }
    }

    /// defines the pointer as u64
    pub fn as_u64(&self) -> u64 {
        let ptr = self.ptr as u64;
        let len = self.len as u64;

        ptr | (len << 32)
    }

    /// allocates the appropriate size of memory
    pub fn allocate(len: u32) -> Self {
        let buffer = alloc::vec![0; len as usize];
        Pointer::release_buffer(buffer.to_vec())
    }

    /// frees the allocated memory
    pub fn deallocate(self) {
        let v = unsafe {
            Vec::from_raw_parts(self.ptr as *mut u8, self.len as usize, self.len as usize)
        };

        core::mem::drop(v);
    }

    /// releases the vector and keep the buffer as an allocated memory.
    /// The memory should be freed later
    pub fn release_buffer(mut buffer: Vec<u8>) -> Self {
        buffer.shrink_to_fit();
        assert!(
            buffer.len() == buffer.capacity(),
            "Buffer is not aligned. len: {}, capacity: {}",
            buffer.len(),
            buffer.capacity()
        );

        let len = u32::try_from(buffer.len()).expect("length doesn't fit in u32");
        let ptr = buffer.as_ptr();

        core::mem::forget(buffer);

        Self { ptr, len }
    }

    /// forms a slice from a pointer and a length.
    /// Warning: only use this when you are sure the caller will never use (or free) the pointer later
    pub unsafe fn to_slice<'a>(&self) -> &'a [u8] {
        core::slice::from_raw_parts(self.ptr, self.len as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_to_slice() {
        let v1 = b"test-vector".to_vec();
        let ptr = Pointer::release_buffer(v1.clone());
        let v2 = unsafe { ptr.to_slice() };

        assert_eq!(v1, v2);
    }

    #[wasm_bindgen_test]
    fn test_allocate() {
        let pointer = Pointer::allocate(1024);
        let ptr_64 = pointer.as_u64();

        let len = (ptr_64 >> 32) as usize;
        let ptr = (ptr_64 & 0xFFFFFFFF) as *mut u8;
        assert_eq!(len, 1024);
        assert!(!ptr.is_null());
    }

    #[wasm_bindgen_test]
    fn test_from_u64() {
        let ptr_64: u64 = 0x0123456789abcdef;
        let ptr = Pointer::from_u64(ptr_64);

        assert_eq!(ptr.len, 0x01234567);
        assert_eq!(ptr.ptr as u32, 0x89abcdef);
    }
}
