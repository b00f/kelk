//! Kelk TODO

use minicbor_derive::Encode;

/// TODO
#[derive(Debug, Encode)]
pub struct Response {
    // TODO: vec<u8> is a bad practice. look for a better response structure
    // data: Vec<u8>
    /// TODO
    #[n(0)]
    pub res: i32,
}
