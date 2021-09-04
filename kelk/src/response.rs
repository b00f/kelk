//! Kelk TODO

use serde::Serialize;

/// TODO
#[derive(Debug, Serialize)]
pub struct Response {
    // TODO: vec<u8> is a bad practice. look for a better response structure
    // data: Vec<u8>
    /// TODO
    pub res: i32,
}
