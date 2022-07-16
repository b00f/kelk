use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum Error {
    #[n(0)]
    StorageError,
}

// TODO: error message
impl From<kelk::storage::error::Error> for Error {
    fn from(_error: kelk::storage::error::Error) -> Self {
        Error::StorageError
    }
}
