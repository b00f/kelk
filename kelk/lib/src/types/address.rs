//! Address type for representing actor address
use core::result::Result;

/// Address type
const ADDRESS_SIZE: usize = 21;

/// Address type in Zarb blockchain
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address([u8; ADDRESS_SIZE]);

// impl Address {
//     pub fn from_bytes(buf: &[u8]) -> Result<Self> {
//         let data = buf.try_into().map_err(|_| Error::InvalidLength {
//             expected: ADDRESS_SIZE,
//             found: buf.len(),
//         })?;
//         Ok(Self(data))
//     }

//     pub fn as_bytes(&self) -> &[u8; ADDRESS_SIZE] {
//         &self.0
//     }
// }

impl<C> minicbor::Encode<C> for Address {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        e.bytes(self.0.as_ref())?;
        Ok(())
    }
}

impl<'a, C> minicbor::Decode<'a, C> for Address {
    fn decode(
        d: &mut minicbor::Decoder<'a>,
        _ctx: &mut C,
    ) -> core::result::Result<Address, minicbor::decode::Error> {
        let buf = d.bytes()?;
        let data = buf
            .try_into()
            .map_err(|_| minicbor::decode::Error::message("invalid data"))?;

        Ok(Address(data))
    }
}

#[cfg(test)]
mod tests {
    // pub fn generate_test_address() -> Address {}
}
