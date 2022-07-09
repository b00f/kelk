use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum Error {
    #[n(0)]
    KelkError,
    #[n(1)]
    DivByZero,
}
