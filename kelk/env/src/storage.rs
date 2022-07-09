//! Storage trait to read and write primitives

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::error::Error;
use ::core::result::Result;
use core::mem::{self, size_of};
use core::slice;
use core::str::from_utf8;

macro_rules! impl_num {
    ($ty:ty, $size:literal, $read_fn:ident, $write_fn:ident) => {
        doc_comment! {
            concat!("reads ", stringify!($size), " byte(s) from storage file at the given offset and converts it to ", stringify!($ty),"."
            ),
            #[inline]
            pub fn $read_fn(&self, offset: u32) -> Result<$ty, Error> {
                Ok(<$ty>::from_be_bytes(
                    self.read(offset, $size)?.try_into().unwrap(),
                ))
            }
        }

        doc_comment! {
                concat!("converts ", stringify!($ty)," to ", stringify!($size), " byte(s) and writes into storage file at the given offset."
                ),
            #[inline]
            pub fn $write_fn(&self, offset: u32, value: $ty) -> Result<(), Error> {
                self.write(offset, &value.to_be_bytes())
            }
        }
    };
}

/// the storage APIs that provided by the host
pub trait StorageAPI {
    /// writes `data` into the storage file at the given offset
    fn read(&self, offset: u32, len: u32) -> Result<Vec<u8>, Error>;

    /// reads `data` from the storage file at the given offset and length
    fn write(&self, offset: u32, data: &[u8]) -> Result<(), Error>;
}

/// Storage object
pub struct Storage {
    /// APIs the provided by th host
    pub api: Box<dyn StorageAPI>,
}

impl Storage {
    /// creates a new instance of storage
    pub fn new(api: Box<dyn StorageAPI>) -> Self {
        Self { api }
    }

    impl_num!(u8, 1, read_u8, write_u8);
    impl_num!(u16, 2, read_u16, write_u16);
    impl_num!(u32, 4, read_u32, write_u32);
    impl_num!(u64, 8, read_u64, write_u64);

    impl_num!(i8, 1, read_i8, write_i8);
    impl_num!(i16, 2, read_i16, write_i16);
    impl_num!(i32, 4, read_i32, write_i32);
    impl_num!(i64, 8, read_i64, write_i64);

    /// reads 1 byte from storage file at the given offset and converts it to bool.
    #[inline]
    pub fn read_bool(&self, offset: u32) -> Result<bool, Error> {
        match self.read_i8(offset)? {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// converts bool to 1 byte(s) and writes into storage file at the given offset.
    #[inline]
    pub fn write_bool(&self, offset: u32, value: bool) -> Result<(), Error> {
        match value {
            true => self.write_i8(offset, 1),
            false => self.write_i8(offset, 0),
        }
    }

    /// reads string from the storage file at the given offset.
    #[inline]
    pub fn read_string(&self, offset: u32, max_len: u32) -> Result<String, Error> {
        let data = self.read(offset, max_len)?;
        let mut iter = data.split(|e| e == &0);
        let str = from_utf8(iter.next().unwrap())
            .map_err(|_| Error::GenericError("Invalid utf8 character"))?;
        Ok(str.to_string())
    }

    /// write string to the storage file at the given offset.
    /// if the length string is greater than max_length it wil be truncated to the max length.
    #[inline]
    pub fn write_string(&self, offset: u32, value: &str, max_len: u32) -> Result<(), Error> {
        let mut data = value.as_bytes();

        if data.len() > max_len as usize {
            data = &data[0..max_len as usize];
        }
        self.write(offset, data)
    }

    /// reads struct T from the storage file at the given offset
    pub fn read_struct<T: Sized>(&self, offset: u32) -> Result<T, Error> {
        let data = self.read(offset, size_of::<T>() as u32)?;
        Ok(unsafe { core::ptr::read(data.as_ptr() as *const _) })
    }

    /// writes struct T to the storage file at the given offset
    pub fn write_struct<T: Sized>(&self, offset: u32, st: &T) -> Result<(), Error> {
        let p: *const T = st;
        let p: *const u8 = p as *const u8; // convert between pointer types
        let b = unsafe { slice::from_raw_parts(p, mem::size_of::<T>()) };

        self.write(offset, b)
    }

    /// writes `data` into the storage file at the given offset
    pub fn read(&self, offset: u32, len: u32) -> Result<Vec<u8>, Error> {
        self.api.read(offset, len)
    }

    /// reads `data` from the storage file at the given offset and length
    pub fn write(&self, offset: u32, data: &[u8]) -> Result<(), Error> {
        self.api.write(offset, data)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::mock_storage;

    #[test]
    fn test_negative_integers() {
        let mock = mock_storage(15);

        mock.write_i8(0, -1).unwrap();
        mock.write_i16(1, -2).unwrap();
        mock.write_i32(3, -3).unwrap();
        mock.write_i64(7, -4).unwrap();

        assert_eq!(mock.read_i8(0).unwrap(), -1);
        assert_eq!(mock.read_i16(1).unwrap(), -2);
        assert_eq!(mock.read_i32(3).unwrap(), -3);
        assert_eq!(mock.read_i64(7).unwrap(), -4);
    }

    #[test]
    fn test_unsigned_integers() {
        let mock = mock_storage(15);

        mock.write_u8(0, 1).unwrap();
        mock.write_u16(1, 2).unwrap();
        mock.write_u32(3, 3).unwrap();
        mock.write_u64(7, 4).unwrap();

        assert_eq!(mock.read_u8(0).unwrap(), 1);
        assert_eq!(mock.read_u16(1).unwrap(), 2);
        assert_eq!(mock.read_u32(3).unwrap(), 3);
        assert_eq!(mock.read_u64(7).unwrap(), 4);
    }

    #[test]
    fn test_bool() {
        let mock = mock_storage(1);

        mock.write_bool(0, true).unwrap();
        assert!(mock.read_bool(0).unwrap());
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, PartialEq)]
        struct Test {
            foo: i16,
            bar: i8,
            zoo: i32,
        }

        let storage = mock_storage(64);
        let foo_1 = Test {
            foo: 123,
            bar: 7,
            zoo: 1024,
        };

        storage.write_struct::<Test>(13, &foo_1).unwrap();
        let foo_2 = storage.read_struct::<Test>(13).unwrap();
        assert_eq!(foo_1, foo_2);
    }
}
