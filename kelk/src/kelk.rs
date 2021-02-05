use super::buffer::{ScopedBuffer, StaticBuffer};
use super::error::Error;
use super::ptr::{Ptr32, Ptr32Mut};
use super::sys;
use super::Key;

fn extract_from_slice(output: &mut &mut [u8], new_len: usize) {
    debug_assert!(new_len <= output.len());
    let tmp = core::mem::take(output);
    *output = &mut tmp[..new_len];
}

pub struct Kelk {
    /// Encode & decode buffer with static size of 4kB.
    ///
    buffer: StaticBuffer,
}

impl Kelk {
    pub fn new() -> Self {
        Kelk {
            buffer: StaticBuffer::new(),
        }
    }
    /// Returns a new scoped buffer for the entire scope of the static 16kB buffer.
    fn scoped_buffer(&mut self) -> ScopedBuffer {
        ScopedBuffer::from(&mut self.buffer[..])
    }

    pub fn set_storage<V>(&mut self, key: &Key, value: &V)
    where
        V: scale::Encode,
    {
        let buffer = self.scoped_buffer().take_encoded(value);

        unsafe { sys::zarb_set_storage(key, Ptr32::from_slice(buffer), buffer.len() as u32) }
    }

    pub fn get_storage<R>(&mut self, key: &Key) -> Option<R>
    where
        R: scale::Decode,
    {
        let output = &mut self.scoped_buffer().take_rest();

        let mut output_len = output.len() as u32;
        let ret_code = {
            unsafe {
                sys::zarb_get_storage(
                    key,
                    Ptr32Mut::from_slice(output),
                    Ptr32Mut::from_ref(&mut output_len),
                )
            }
        };
        match ret_code.into() {
            Ok(_) => (),
            Err(Error::KeyNotFound) => return None,
            Err(_) => panic!("encountered unexpected error"),
        };

        extract_from_slice(output, output_len as usize);
        let decoded = scale::Decode::decode(&mut &output[..]).unwrap();
        Some(decoded)
    }

    pub fn println(&mut self, content: &str) {
        let bytes = content.as_bytes();
        unsafe { sys::zarb_println(Ptr32::from_slice(bytes), bytes.len() as u32) }
    }
}
