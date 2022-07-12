//! The Blockchain APIs for interacting with blockchain

use alloc::boxed::Box;
use kelk_env::BlockchainAPI;

/// Blockchain object
pub struct Blockchain {
    /// APIs the provided by th host
    pub api: Box<dyn BlockchainAPI>,
}

impl Blockchain {
    /// creates a new instance of Blockchain
    pub fn new(api: Box<dyn BlockchainAPI>) -> Self {
        Self { api }
    }
}
