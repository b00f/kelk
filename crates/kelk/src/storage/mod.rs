//! Modules definition for storage libraries

pub mod bst;
pub mod codec;
pub mod error;
pub mod linked_list;
pub mod mock;
pub mod str;
pub mod vec;

/// is an alias for representing the offset of the allocated space inside the storage file.
pub type Offset = u32;

use self::codec::Codec;
use self::error::Error;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::result::Result;
use kelk_env::StorageAPI;

macro_rules! impl_num {
    ($ty:ty, $size:literal, $read_fn:ident, $write_fn:ident) => {
        doc_comment! {
            concat!("reads ", stringify!($size), " byte(s) from storage file at the given `offset` and converts it to ", stringify!($ty),"."
            ),
            #[inline]
            pub fn $read_fn(&self, offset: Offset) -> Result<$ty,Error> {
                self.read::<$ty>(offset)
            }
        }

        doc_comment! {
                concat!("converts ", stringify!($ty)," to ", stringify!($size), " byte(s) and writes it into storage file at the given `offset`."
                ),
            #[inline]
            pub fn $write_fn(&self, offset: Offset, value: &$ty) -> Result<(),Error> {
                self.write(offset, value)
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
        api.write(0, &[1, 0])?; // version = 1
        api.write(2, &[0, 1])?; // stack size = 256
        api.write(4, &[0; 256 * 4])?; // stack
        api.write(1028, &[0, 0, 4, 8])?; // free storage pos

        let storage = Storage {
            api,
            stack_size: 256,
        };
        // let freed = StorageLinkedList::create(&storage, 0)?;
        // storage.freed = Some(freed);

        Ok(storage)
    }

    ///
    pub fn load(api: Box<dyn StorageAPI>) -> Result<Self, Error> {
        let ver = api.read(0, 2)?;
        let stack_size = api.read(2, 2)?;
        if !ver.eq(&[1, 0]) || !stack_size.eq(&[0, 1]) {
            return Err(Error::GenericError("invalid storage file".to_string()));
        }

        let storage = Storage {
            api,
            stack_size: 256,
        };

        Ok(storage)
    }

    pub(crate) fn api_mut(&mut self) -> &mut Box<dyn StorageAPI> {
        &mut self.api
    }

    ///
    pub fn allocate(&self, length: usize) -> Result<Offset, Error> {
        let cur_free_pos = self.read_u32(1028)?;
        let next_free_pos = cur_free_pos + length as u32;

        // Updating allocation pos
        self.write_u32(1028, &next_free_pos)?;

        Ok(cur_free_pos)
    }

    fn stack_offset(&self, stack_index: u16) -> Result<Offset, Error> {
        if stack_index > self.stack_size {
            return Err(Error::StackOverflow);
        }

        // stack_offset = (stack_index * 4) + 4
        let header_size = 4;
        Ok(((stack_index as usize * Offset::PACKED_LEN) + header_size) as Offset)
    }

    ///
    pub fn fill_stack_at(&self, stack_index: u16, offset: Offset) -> Result<(), Error> {
        self.write_u32(self.stack_offset(stack_index)?, &offset)
    }

    ///
    pub fn read_stack_at(&self, stack_index: u16) -> Result<Offset, Error> {
        self.read_u32(self.stack_offset(stack_index)?)
    }

    impl_num!(u8, 1, read_u8, write_u8);
    impl_num!(u16, 2, read_u16, write_u16);
    impl_num!(u32, 4, read_u32, write_u32);
    impl_num!(u64, 8, read_u64, write_u64);
    impl_num!(u128, 8, read_u128, write_u128);

    impl_num!(i8, 1, read_i8, write_i8);
    impl_num!(i16, 2, read_i16, write_i16);
    impl_num!(i32, 4, read_i32, write_i32);
    impl_num!(i64, 8, read_i64, write_i64);
    impl_num!(i128, 16, read_i128, write_i128);

    impl_num!(bool, 1, read_bool, write_bool);

    /// Reads `T` from the storage file at the given `offset`.
    /// Note that `T` should be `Codec`.
    #[inline]
    pub(crate) fn read<T: Codec>(&self, offset: u32) -> Result<T, Error> {
        let data = self.api.read(offset, T::PACKED_LEN as u32)?;
        let value = T::from_bytes(&data);
        Ok(value)
    }

    /// Writes `T` to the storage file at the given `offset`.
    /// Note that `T` should be `Codec`.
    #[inline]
    pub(crate) fn write<T: Codec>(&self, offset: Offset, value: &T) -> Result<(), Error> {
        let data = value.to_bytes();
        Ok(self.api.write(offset, &data)?)
    }

    /// Reads slice of bytes of size `length` from the storage file at the given `offset`.
    #[inline]
    pub(crate) fn read_bytes(&self, offset: u32, length: u32) -> Result<Vec<u8>, Error> {
        Ok(self.api.read(offset, length)?)
    }

    /// Writes bytes slice to the storage file at the given `offset`.
    #[inline]
    pub(crate) fn write_bytes(&self, offset: Offset, data: &[u8]) -> Result<(), Error> {
        Ok(self.api.write(offset, data)?)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::storage::codec::Codec;
    use crate::storage::mock::mock_storage;
    use alloc::vec::Vec;
    use kelk_derive::Codec;

    #[test]
    fn test_unsigned_integers() {
        let storage = mock_storage(1024 * 1024);

        let offset1 = storage.allocate(u8::PACKED_LEN).unwrap();
        let offset2 = storage.allocate(u16::PACKED_LEN).unwrap();
        let offset3 = storage.allocate(u32::PACKED_LEN).unwrap();
        let offset4 = storage.allocate(u64::PACKED_LEN).unwrap();
        let offset5 = storage.allocate(u128::PACKED_LEN).unwrap();

        storage.write_u8(offset1, &1).unwrap();
        storage.write_u16(offset2, &2).unwrap();
        storage.write_u32(offset3, &3).unwrap();
        storage.write_u64(offset4, &4).unwrap();
        storage.write_u128(offset5, &5).unwrap();

        assert_eq!(storage.read_u8(offset1).unwrap(), 1);
        assert_eq!(storage.read_u16(offset2).unwrap(), 2);
        assert_eq!(storage.read_u32(offset3).unwrap(), 3);
        assert_eq!(storage.read_u64(offset4).unwrap(), 4);
        assert_eq!(storage.read_u128(offset5).unwrap(), 5);
    }

    #[test]
    fn test_signed_integers() {
        let storage = mock_storage(1024 * 1024);

        let offset1 = storage.allocate(i8::PACKED_LEN).unwrap();
        let offset2 = storage.allocate(i16::PACKED_LEN).unwrap();
        let offset3 = storage.allocate(i32::PACKED_LEN).unwrap();
        let offset4 = storage.allocate(i64::PACKED_LEN).unwrap();
        let offset5 = storage.allocate(i128::PACKED_LEN).unwrap();

        storage.write_i8(offset1, &-1).unwrap();
        storage.write_i16(offset2, &-2).unwrap();
        storage.write_i32(offset3, &-3).unwrap();
        storage.write_i64(offset4, &-4).unwrap();
        storage.write_i128(offset5, &-5).unwrap();

        assert_eq!(storage.read_i8(offset1).unwrap(), -1);
        assert_eq!(storage.read_i16(offset2).unwrap(), -2);
        assert_eq!(storage.read_i32(offset3).unwrap(), -3);
        assert_eq!(storage.read_i64(offset4).unwrap(), -4);
        assert_eq!(storage.read_i128(offset5).unwrap(), -5);
    }

    #[test]
    fn test_bool() {
        let storage = mock_storage(1024 * 1024);

        let offset1 = storage.allocate(bool::PACKED_LEN).unwrap();
        let offset2 = storage.allocate(bool::PACKED_LEN).unwrap();

        storage.write_bool(offset1, &true).unwrap();
        storage.write_bool(offset2, &false).unwrap();

        assert!(storage.read_bool(offset1).unwrap());
        assert!(!storage.read_bool(offset2).unwrap());
    }

    #[test]
    fn test_struct() {
        use self::Codec;

        #[derive(Debug, PartialEq, Codec, Clone)]
        struct Test {
            foo: i16,
            bar: i8,
            zoo: i32,
        }

        let storage = mock_storage(1024 * 1024);
        let foo_1 = Test {
            foo: 123,
            bar: 7,
            zoo: 1024,
        };

        let offset = storage.allocate(Test::PACKED_LEN).unwrap();

        storage.write(offset, &foo_1).unwrap();
        let foo_2 = storage.read::<Test>(offset).unwrap();
        assert_eq!(foo_1, foo_2);
    }
}
