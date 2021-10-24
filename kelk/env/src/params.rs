//!
//!

use minicbor::{Decode, Encode};

///
pub const PARAM_CALLER_ADDRESS: i32 = 0x0010;
///
pub const PARAM_CALLER_ID: i32 = 0x0011;

/// Parameter value types
#[derive(Encode, Decode)]
pub enum ParamType {
    /// A 32-bit integer.
    #[n(0)]
    I32 {
        #[doc(hidden)]
        #[n(0)]
        value: i32,
    },
    /// A 64-bit integer.
    #[n(1)]
    I64 {
        #[doc(hidden)]
        #[n(0)]
        value: i64,
    },
    // #[n(2)]
    // I128 {
    //     #[n(0)]
    //     value: i128,
    // },
    // #[n(10)]
    // Address {
    //     #[n(0)]
    //     value: [u8; 20],
    // },
}
