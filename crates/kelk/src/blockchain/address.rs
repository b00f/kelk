//! Address type for representing actor address
use super::error::Error;
use core::result::Result;

/// Address type
pub const ADDRESS_SIZE: usize = 21;

/// Address type in Zarb blockchain
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address([u8; ADDRESS_SIZE]);

impl Address {
    /// Converting address from byte slice
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        let data = buf.try_into().map_err(|_| Error::InvalidLength {
            expected: ADDRESS_SIZE,
            found: buf.len(),
        })?;
        Ok(Self(data))
    }

    /// Converting address to a fixed byte array
    pub fn as_bytes(&self) -> &[u8; ADDRESS_SIZE] {
        &self.0
    }
}

impl<C> minicbor::Encode<C> for Address {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _: &mut C,
    ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        e.bytes(self.as_bytes())?;
        Ok(())
    }
}

impl<'a, C> minicbor::Decode<'a, C> for Address {
    fn decode(
        d: &mut minicbor::Decoder<'a>,
        _: &mut C,
    ) -> core::result::Result<Address, minicbor::decode::Error> {
        Address::from_bytes(d.bytes()?)
            .map_err(|_| minicbor::decode::Error::message("invalid data"))
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn test_decoding() {
        let addr = Address([
            01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01,
        ]);
        let bytes = [
            0x55, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01,
            01,
        ];
        let encoded_addr = minicbor::to_vec(addr.clone()).unwrap();
        let decoded_addr = minicbor::decode::<Address>(&bytes).unwrap();
        assert_eq!(decoded_addr, addr);
        assert_eq!(encoded_addr, bytes);
    }
}
