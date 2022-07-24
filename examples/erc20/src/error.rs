use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum Error {
    #[n(0)]
    StorageError,
    #[n(1)]
    InvalidMsg,
    #[n(2)]
    InsufficientAmount,
}


// TODO: error message
impl From<kelk::storage::error::Error> for Error {
    fn from(_error: kelk::storage::error::Error) -> Self {
        Error::StorageError
    }
}
