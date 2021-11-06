//! Kelk TODO

use alloc::vec::Vec;

use crate::{
    context::{ContextAPI, OwnedContext},
    error::KelkError,
    params::ParamType,
};

///todo
pub struct MockContext {
    storage: Vec<u8>,
}

///todo
impl MockContext {
    ///todo
    pub fn new(size: usize) -> OwnedContext<MockContext> {
        let mut storage = Vec::with_capacity(size);
        storage.fill(0);
        OwnedContext {
            api: MockContext { storage },
        }
    }
}

impl ContextAPI for MockContext {
    fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        if offset as usize + data.len() > self.storage.len() {
            return Err(KelkError::StorageOutOfBound);
        }
        for i in 0..data.len() - 1 {
            self.storage[i + offset as usize] = data[i];
        }
        Ok(())
    }

    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError> {
        let c = &self.storage[offset as usize..offset as usize + length as usize];
        Ok(c.into())
    }

    fn get_param(&self, _param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!()
    }
}
