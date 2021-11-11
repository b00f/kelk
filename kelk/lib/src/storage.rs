//! Storage trait to read and write primitives

use ::core::result::Result;
use alloc::vec::Vec;

/// A genral list of storage error
#[derive(Debug)]
pub enum Error {
    /// Unexpected End Of File
    UnexpectedEof,
    /// An issue occurred on writing into the storage file
    WriteStorageFailed,
    /// An issue occurred on reading from the storage file
    ReadStorageFailed,
}

macro_rules! impl_num {
    ($ty:ty, $size:literal, $sread_fn:ident, $swrite_fn:ident) => {
        doc_comment! {
            concat!("reads ", stringify!($size), " byte(s) from storage file at the given offset and converts it to ", stringify!($ty),"."
            ),
            #[inline]
            fn $sread_fn(&self, offset: u32) -> Result<$ty, Error> {
                Ok(<$ty>::from_be_bytes(
                    self.sread(offset, $size)?.try_into().unwrap(),
                ))
            }
        }

        doc_comment! {
                concat!("converts ", stringify!($ty)," to ", stringify!($size), " byte(s) and writes into storage file at the given offset."
                ),
            #[inline]
            fn $swrite_fn(&self, offset: u32, value: $ty) -> Result<(), Error> {
                self.swrite(offset, &value.to_be_bytes())
            }
        }
    };
}

/// Storage trait
pub trait Storage {
    impl_num!(u8, 1, sread_u8, swrite_u8);
    impl_num!(u16, 2, sread_u16, swrite_u16);
    impl_num!(u32, 4, sread_u32, swrite_u32);
    impl_num!(u64, 8, sread_u64, swrite_u64);

    impl_num!(i8, 1, sread_i8, swrite_i8);
    impl_num!(i16, 2, sread_i16, swrite_i16);
    impl_num!(i32, 4, sread_i32, swrite_i32);
    impl_num!(i64, 8, sread_i64, swrite_i64);

    /// reads 1 byte from storage file at the given offset and converts it to bool.
    #[inline]
    fn sread_bool(&self, offset: u32) -> Result<bool, Error> {
        match self.sread_i8(offset)? {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// converts bool to 1 byte(s) and writes into storage file at the given offset.
    #[inline]
    fn swrite_bool(&self, offset: u32, value: bool) -> Result<(), Error> {
        match value {
            true => self.swrite_i8(offset, 1),
            false => self.swrite_i8(offset, 0),
        }
    }

    /// writes `data` into the storage file at the given offset
    fn sread(&self, offset: u32, len: u32) -> Result<Vec<u8>, Error>;

    /// reads `data` from the storage file at the given offset and length
    fn swrite(&self, offset: u32, data: &[u8]) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::mock_storage;

    #[test]
    fn test_negative_integers() {
        let mock = mock_storage(15);

        mock.swrite_i8(0, -1).unwrap();
        mock.swrite_i16(1, -2).unwrap();
        mock.swrite_i32(3, -3).unwrap();
        mock.swrite_i64(7, -4).unwrap();

        assert_eq!(mock.sread_i8(0).unwrap(), -1);
        assert_eq!(mock.sread_i16(1).unwrap(), -2);
        assert_eq!(mock.sread_i32(3).unwrap(), -3);
        assert_eq!(mock.sread_i64(7).unwrap(), -4);
    }

    #[test]
    fn test_unsigned_integers() {
        let mock = mock_storage(15);

        mock.swrite_u8(0, 1).unwrap();
        mock.swrite_u16(1, 2).unwrap();
        mock.swrite_u32(3, 3).unwrap();
        mock.swrite_u64(7, 4).unwrap();

        assert_eq!(mock.sread_u8(0).unwrap(), 1);
        assert_eq!(mock.sread_u16(1).unwrap(), 2);
        assert_eq!(mock.sread_u32(3).unwrap(), 3);
        assert_eq!(mock.sread_u64(7).unwrap(), 4);
    }

    #[test]
    fn test_bool() {
        let mock = mock_storage(1);

        mock.swrite_bool(0, true).unwrap();
        assert!(mock.sread_bool(0).unwrap());
    }
}
