//! Storage trait to read and write primitives

use super::allocated::Allocated;
use super::error::Error;
use super::Offset;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem::{self, size_of};
use core::result::Result;
use core::slice;
use core::str::from_utf8;
use kelk_env::StorageAPI;

macro_rules! impl_num {
    ($ty:ty, $size:literal, $read_fn:ident, $write_fn:ident) => {
        doc_comment! {
            concat!("reads ", stringify!($size), " byte(s) from storage file at the given `offset` and converts it to ", stringify!($ty),"."
            ),
            #[inline]
            pub fn $read_fn(&self, offset: u32) -> Result<$ty,Error> {
                Ok(<$ty>::from_be_bytes(
                    self.read(offset, $size)?.try_into().unwrap(),
                ))
            }
        }

        doc_comment! {
                concat!("converts ", stringify!($ty)," to ", stringify!($size), " byte(s) and writes it into storage file at the given `offset`."
                ),
            #[inline]
            pub fn $write_fn(&self, offset: u32, value: $ty) -> Result<(),Error> {
                self.write(offset, &value.to_be_bytes())
            }
        }
    };
}

/// Storage object
pub struct Storage {
    /// Storage APIs that are provided by the host
    api: Box<dyn StorageAPI>,

    stack_size: u16,
}

impl Storage {
    /// creates a new instance of storage
    pub fn create(api: Box<dyn StorageAPI>) -> Result<Self, Error> {
        api.write(0, &[1])?; // version = 1
        api.write(1, &[0])?; // reserved = 0
        api.write(2, &[1, 0])?; // stack size = 256
        api.write(4, &[0; 256 * 4])?; // stack
        api.write(1028, &[0, 0, 4, 8])?; // free storage pos

        let mut storage = Storage {
            api,
            stack_size: 256,
        };
        // let freed = StorageLinkedList::create(&storage, 0)?;
        // storage.freed = Some(freed);

        Ok(storage)
    }

    ///
    pub fn load(api: Box<dyn StorageAPI>) -> Result<Self, Error> {
        let ver = api.read(0, 1)?;
        let res = api.read(1, 1)?;
        let stack_size = api.read(2, 2)?;
        if !ver.eq(&[1]) || !res.eq(&[0]) || !stack_size.eq(&[1, 0]) {
            return Err(Error::GenericError("invalid storage file".to_string()));
        }

        let mut storage = Storage {
            api,
            stack_size: 256,
        };

        Ok(storage)
    }

    pub(crate) fn api_mut(&mut self) -> &mut Box<dyn StorageAPI> {
        &mut self.api
    }

    ///
    pub fn allocate<T: Sized>(&self, data: T) -> Result<Allocated<T>, Error> {
        let free_pos = self.read_u32(1028)?;
        let allocated = Allocated::new(free_pos, data, false);
        self.write_u32(1028, free_pos + size_of::<T>() as u32)?;
        Ok(allocated)
    }

    fn stack_offset(&self, stack_index: u16) -> Result<Offset, Error> {
        if stack_index > self.stack_size {
            return Err(Error::StackOverflow);
        }

        // stack_offset = (stack_index * 4) + 4
        let header_size = 4;
        Ok(((stack_index as usize * size_of::<Offset>()) + header_size) as Offset)
    }

    pub(crate) fn fill_stack_at(&self, stack_index: u16, offset: Offset) -> Result<(), Error> {
        self.write_u32(self.stack_offset(stack_index)?, offset)
    }

    pub(crate) fn read_stack_at(&self, stack_index: u16) -> Result<Offset, Error> {
        self.read_u32(self.stack_offset(stack_index)?)
    }

    impl_num!(u8, 1, read_u8, write_u8);
    impl_num!(u16, 2, read_u16, write_u16);
    impl_num!(u32, 4, read_u32, write_u32);
    impl_num!(u64, 8, read_u64, write_u64);

    impl_num!(i8, 1, read_i8, write_i8);
    impl_num!(i16, 2, read_i16, write_i16);
    impl_num!(i32, 4, read_i32, write_i32);
    impl_num!(i64, 8, read_i64, write_i64);

    /// reads 1 byte from storage file at the given `offset` and converts it to bool.
    #[inline]
    pub(crate) fn read_bool(&self, offset: u32) -> Result<bool, Error> {
        match self.read_i8(offset)? {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// converts bool to 1 byte and writes it into storage file at the given `offset`.
    #[inline]
    pub(crate) fn write_bool(&self, offset: u32, value: bool) -> Result<(), Error> {
        match value {
            true => self.write_i8(offset, 1),
            false => self.write_i8(offset, 0),
        }
    }

    /// reads string from the storage file at the given `offset`.
    #[inline]
    pub(crate) fn read_string(&self, offset: u32, max_len: u32) -> Result<String, Error> {
        let data = self.read(offset, max_len)?;
        let mut iter = data.split(|e| e == &0);
        let str =
            from_utf8(iter.next().unwrap()).map_err(|err| Error::GenericError(err.to_string()))?;
        Ok(str.to_string())
    }

    /// write string to the storage file at the given `offset`.
    /// if the length string is greater than `max_len` it wil be truncated to the `max_len`.
    #[inline]
    pub fn write_string(&self, offset: u32, value: &str, max_len: u32) -> Result<(), Error> {
        let mut data = value.as_bytes().to_vec();
        data.resize(max_len as usize, 0);

        self.write(offset, &data)
    }

    /// reads struct `T` from the storage file at the given `offset`.
    /// Note that struct `T` should be `Sized`.
    #[inline]
    pub(crate) fn read_struct<T: Sized>(&self, offset: u32) -> Result<Allocated<T>, Error> {
        let data = self.read(offset, size_of::<T>() as u32)?;
        Ok(unsafe { core::ptr::read(data.as_ptr() as *const _) })
    }

    /// writes struct `T` to the storage file at the given `offset`.
    /// Note that struct `T` should be `Sized`.
    #[inline]
    pub(crate) fn write_struct<T: Sized>(&self, offset: u32, st: &T) -> Result<(), Error> {
        let p: *const T = st;
        let p: *const u8 = p as *const u8; // convert between pointer types
        let b = unsafe { slice::from_raw_parts(p, mem::size_of::<T>()) };

        self.write(offset, b)
    }

    /// reads bytes from the storage file at the given `offset` up to the given `length`.
    #[inline]
    pub(crate) fn read(&self, offset: u32, len: u32) -> Result<Vec<u8>, Error> {
        Ok(self.api.read(offset, len)?)
    }

    /// writes `data` into the storage file at the given `offset`.
    #[inline]
    pub(crate) fn write(&self, offset: u32, data: &[u8]) -> Result<(), Error> {
        Ok(self.api.write(offset, data)?)
    }
}

// #[cfg(test)]
// pub mod tests {
//     use alloc::string::ToString;

//     use crate::storage::mock::mock_storage;

//     #[test]
//     fn test_negative_integers() {
//         let mock = mock_storage(15);

//         mock.write_i8(0, -1).unwrap();
//         mock.write_i16(1, -2).unwrap();
//         mock.write_i32(3, -3).unwrap();
//         mock.write_i64(7, -4).unwrap();

//         assert_eq!(mock.read_i8(0).unwrap(), -1);
//         assert_eq!(mock.read_i16(1).unwrap(), -2);
//         assert_eq!(mock.read_i32(3).unwrap(), -3);
//         assert_eq!(mock.read_i64(7).unwrap(), -4);
//     }

//     #[test]
//     fn test_unsigned_integers() {
//         let mock = mock_storage(15);

//         mock.write_u8(0, 1).unwrap();
//         mock.write_u16(1, 2).unwrap();
//         mock.write_u32(3, 3).unwrap();
//         mock.write_u64(7, 4).unwrap();

//         assert_eq!(mock.read_u8(0).unwrap(), 1);
//         assert_eq!(mock.read_u16(1).unwrap(), 2);
//         assert_eq!(mock.read_u32(3).unwrap(), 3);
//         assert_eq!(mock.read_u64(7).unwrap(), 4);
//     }

//     #[test]
//     fn test_bool() {
//         let mock = mock_storage(1);

//         mock.write_bool(0, true).unwrap();
//         assert!(mock.read_bool(0).unwrap());
//     }

//     #[test]
//     fn test_struct() {
//         #[derive(Debug, PartialEq)]
//         struct Test {
//             foo: i16,
//             bar: i8,
//             zoo: i32,
//         }

//         let storage = mock_storage(64);
//         let foo_1 = Test {
//             foo: 123,
//             bar: 7,
//             zoo: 1024,
//         };

//         storage.write_struct::<Test>(13, &foo_1).unwrap();
//         let foo_2 = storage.read_struct::<Test>(13).unwrap();
//         assert_eq!(foo_1, foo_2);
//     }

//     #[test]
//     fn test_sting() {
//         let storage = mock_storage(64);
//         storage.write_string(0, "foooo", 16).unwrap();
//         storage.write_string(0, "foo", 16).unwrap();
//         let expected = storage.read_string(0, 16).unwrap();
//         assert_eq!(expected, "foo".to_string())
//     }
// }
