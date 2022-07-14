//! Address type for representing actor address
use super::error::Error;
use core::result::Result;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

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

// TODO: Can we move it into mock?
/// generates a random address for testing
pub fn generate_test_address(seed: u64) -> Address {
    let mut small_rng = SmallRng::seed_from_u64(seed);
    let mut buf = [0u8; ADDRESS_SIZE];
    small_rng.fill(&mut buf);
    Address::from_bytes(&buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{generate_test_address, Address};

    #[test]
    fn test_decoding() {
        let addr1 = generate_test_address(0);
        let bytes = minicbor::to_vec(addr1.clone()).unwrap();
        let addr2 = minicbor::decode::<Address>(&bytes).unwrap();
        assert_eq!(addr1, addr2);
    }
}
