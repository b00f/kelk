//! Utilities for encoding multi-byte values
//!
//! In Kelk all multi-byte values are encoded in network byte order
//! (that is, most significant byte first, also known as "big-endian").
//!

use alloc::vec::Vec;

/// `Codec` trait defines functions to serialize types as bytes and deserialize from bytes
/// in big-endian (network) byte order.
pub trait Codec {
    /// Represent the size of packed bytes in big-endian (network) byte order.
    const PACKED_LEN: usize;

    /// Returns the memory representation of this type as a byte array in big-endian (network) byte order.
    fn to_bytes(&self) -> Vec<u8>;

    /// Creates a native endian value from its representation as a byte array in big-endian (network) byte order.
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_codec_for_integer {
    ($type:ty, $size:expr) => {
        impl Codec for $type {
            const PACKED_LEN: usize = $size;

            #[inline]
            fn to_bytes(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Self {
                let arr = bytes.try_into().expect("invalid data");
                Self::from_be_bytes(arr)
            }
        }
    };
}

macro_rules! impl_codec_for_array {
    ($type:ty, $size:expr) => {
        impl Codec for $type {
            const PACKED_LEN: usize = $size;

            #[inline]
            fn to_bytes(&self) -> Vec<u8> {
                self.to_vec()
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::PACKED_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }
    };
}

impl_codec_for_integer!(u8, 1);
impl_codec_for_integer!(i8, 1);
impl_codec_for_integer!(u16, 2);
impl_codec_for_integer!(i16, 2);
impl_codec_for_integer!(u32, 4);
impl_codec_for_integer!(i32, 4);
impl_codec_for_integer!(u64, 8);
impl_codec_for_integer!(i64, 8);
impl_codec_for_integer!(u128, 16);
impl_codec_for_integer!(i128, 16);

impl_codec_for_array!([u8; 1], 1);
impl_codec_for_array!([u8; 2], 2);
impl_codec_for_array!([u8; 3], 3);
impl_codec_for_array!([u8; 4], 4);
impl_codec_for_array!([u8; 5], 5);
impl_codec_for_array!([u8; 6], 6);
impl_codec_for_array!([u8; 7], 7);
impl_codec_for_array!([u8; 8], 8);
impl_codec_for_array!([u8; 9], 9);
impl_codec_for_array!([u8; 10], 10);
impl_codec_for_array!([u8; 11], 11);
impl_codec_for_array!([u8; 12], 12);
impl_codec_for_array!([u8; 13], 13);
impl_codec_for_array!([u8; 14], 14);
impl_codec_for_array!([u8; 15], 15);
impl_codec_for_array!([u8; 16], 16);
impl_codec_for_array!([u8; 17], 17);
impl_codec_for_array!([u8; 18], 18);
impl_codec_for_array!([u8; 19], 19);
impl_codec_for_array!([u8; 20], 20);
impl_codec_for_array!([u8; 21], 21);
impl_codec_for_array!([u8; 22], 22);
impl_codec_for_array!([u8; 23], 23);
impl_codec_for_array!([u8; 24], 24);
impl_codec_for_array!([u8; 25], 25);
impl_codec_for_array!([u8; 26], 26);
impl_codec_for_array!([u8; 27], 27);
impl_codec_for_array!([u8; 28], 28);
impl_codec_for_array!([u8; 29], 29);
impl_codec_for_array!([u8; 30], 30);
impl_codec_for_array!([u8; 31], 31);
impl_codec_for_array!([u8; 32], 32);

impl Codec for bool {
    const PACKED_LEN: usize = 1;

    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            true => [1].to_vec(),
            false => [0].to_vec(),
        }
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        !matches!(bytes[0], 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Codec;

    #[test]
    fn codec_integer() {
        let v1 = 0xabcdef;
        let v2 = 0xabcdefabcdef;
        assert_eq!(i32::from_bytes(&v1.to_bytes()), v1);
        assert_eq!(i64::from_bytes(&v2.to_bytes()), v2);
    }

    #[test]
    fn codec_array() {
        let v1 = [0, 1, 2, 3, 4, 5];
        let v2 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(<[u8; 6]>::from_bytes(&v1.to_bytes()), v1);
        assert_eq!(<[u8; 12]>::from_bytes(&v2.to_bytes()), v2);
    }

    #[test]
    fn codec_struct() {
        #[derive(Codec, PartialEq, Eq, Debug)]
        struct Foo {
            a: u16,
            b: [u8; 3],
        }

        let foo = Foo {
            a: 32,
            b: [1, 2, 3],
        };

        let bytes = foo.to_bytes();
        assert_eq!(foo, Foo::from_bytes(&bytes));
    }
}
