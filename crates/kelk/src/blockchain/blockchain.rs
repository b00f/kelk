//! The Blockchain APIs for interacting with blockchain

use super::address::Address;
use super::error::Error;
use alloc::boxed::Box;
use alloc::vec::Vec;
use kelk_env::BlockchainAPI;

const PARAM_ID_LAST_BLOCK_HASH: u32 = 0x0001;
const PARAM_ID_LAST_BLOCK_TIME: u32 = 0x0002;
const PARAM_ID_TRANSACTION_SIGNER: u32 = 0x0010;

/// Blockchain object
pub struct Blockchain {
    /// APIs the provided by th host
    pub(crate) api: Box<dyn BlockchainAPI>,
}

impl Blockchain {
    /// creates a new instance of Blockchain
    pub fn new(api: Box<dyn BlockchainAPI>) -> Self {
        Self { api }
    }

    /// returns the last block hash
    pub fn get_last_block_hash(&self) -> Result<Vec<u8>, Error> {
        Ok(self.api.get_param(PARAM_ID_LAST_BLOCK_HASH)?)
    }

    /// returns the last block time
    pub fn get_last_block_time(&self) -> Result<Vec<u8>, Error> {
        Ok(self.api.get_param(PARAM_ID_LAST_BLOCK_TIME)?)
    }

    /// returns the transaction signer address
    pub fn get_transaction_signer(&self) -> Result<Address, Error> {
        let data = self.api.get_param(PARAM_ID_TRANSACTION_SIGNER)?;
        Address::from_bytes(&data)
    }
}
