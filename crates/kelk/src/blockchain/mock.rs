//! Mocking the blockchain for testing purpose

use alloc::{collections::BTreeMap, boxed::Box};
use alloc::vec::Vec;
use core::result::Result;
use kelk_env::{BlockchainAPI, HostError};

use super::Blockchain;

/// mocks the blockchain for testing purpose.
pub struct MockBlockchain {
    map: BTreeMap<u32, Vec<u8>>,
}

impl MockBlockchain {
    /// instantiates a new blockchain mock
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl Default for MockBlockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainAPI for MockBlockchain {
    fn get_param<'a>(&self, param_id: u32) -> Result<Vec<u8>, HostError> {
        Ok(self.map.get(&param_id).unwrap().to_vec())
    }
}

/// mocks the blockchain for testing
pub fn mock_blockchain() -> Blockchain {
    let blockchain = MockBlockchain::new();
    Blockchain {
        api: Box::new(blockchain),
    }
}
