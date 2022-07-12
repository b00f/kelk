//! Mocking the blockchain for testing purpose

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::result::Result;
use kelk_env::{BlockchainAPI, Error};

use super::Blockchain;

/// mocks the blockchain for testing purpose.
pub struct MockBlockchain {}

impl MockBlockchain {
    /// instantiates a new blockchain mock
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MockBlockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainAPI for MockBlockchain {
    fn get_param<'a>(&self, _param_id: u32) -> Result<Vec<u8>, Error> {
        todo!()
    }
}

/// mocks the blockchain for testing
pub fn mock_blockchain() -> Blockchain {
    let blockchain = MockBlockchain::new();
    Blockchain {
        api: Box::new(blockchain),
    }
}
