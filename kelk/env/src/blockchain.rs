//! The Blockchain APIs for interacting with blockchain

use crate::params::*;

/// `ContextAPI` provides the storage and blockchain APIs.
/// It can't be copied or cloned since it doesn't have Copy and Clone traits.
pub trait Blockchain {
    /// TODO move it to lib crate
    /// gets the parameter value
    fn get_param(&self, param_id: i32) -> Option<ParamType>;
}
