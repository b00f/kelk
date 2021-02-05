/// The raw return code returned by the host side.
#[repr(transparent)]
pub struct ReturnCode(u32);

pub enum Error {
    KeyNotFound = 1,
    UnknownError,
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<ReturnCode> for Result<()> {
    #[inline]
    fn from(return_code: ReturnCode) -> Self {
        match return_code.0 {
            0 => Ok(()),
            1 => Err(Error::KeyNotFound),
            _ => Err(Error::UnknownError),
        }
    }
}
