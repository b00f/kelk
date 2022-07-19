//! Storage Linked List, is a singly linked list that instead of using Random Access Memory (RAM),
//! it uses storage file. Therefore it's permanently store inside contract's storage.

use super::header::Header;
use super::item::Item;
use crate::storage::error::Error;
use crate::storage::Storage;
use core::marker::PhantomData;
use core::mem::size_of;
use core::result::Result;

/// The instance of Storage Linked List
pub struct StorageLinkedList<'a, I>
where
    I: Sized,
{
    storage: &'a Storage,
    offset: u32,
    header: Header,
    _phantom: PhantomData<I>,
}

impl<'a, I> StorageLinkedList<'a, I>
where
    I: Sized,
{
    /// creates and store a new instance of Storage Linked List Tree at the given offset
    pub fn create(storage: &'a Storage, offset: u32, capacity: u32) -> Result<Self, Error> {
        let header = Header::new::<I>(capacity);
        storage.write_struct::<Header>(offset, &header)?;

        Ok(StorageLinkedList {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }

    /// load the Storage Linked List Tree
    pub fn lazy_load(storage: &'a Storage, offset: u32) -> Result<Self, Error> {
        let header: Header = storage.read_struct(offset)?;

        // TODO:
        // Check boom and reserved field to be correct

        if header.item_len != size_of::<I>() as u16 {
            return Err(Error::InvalidOffset(offset));
        }

        Ok(StorageLinkedList {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }

    /// Inserts an item at the end of linked list.
    pub fn insert(&mut self, item: I) -> Result<Option<I>, Error> {
        if self.header.count == 0 {
            // create a root node
            let root = Item::new(item);
            let head_offset = self.offset + size_of::<Header>() as u32;

            self.header.count = 1;
            self.header.head_offset = head_offset;
            self.header.tail_offset = head_offset;

            self.storage.write_struct(self.offset, &self.header)?;
            self.storage.write_struct(head_offset, &root)?;
            Ok(None)
        } else if self.header.count >= self.header.capacity {
            Err(Error::OutOfCapacity)
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {}
