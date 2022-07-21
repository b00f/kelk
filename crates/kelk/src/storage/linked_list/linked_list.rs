//! Storage Linked List, is a singly linked list that instead of using Random Access Memory (RAM),
//! it uses storage file. Therefore it's permanently store inside contract's storage.

use super::header::Header;
use super::node::{self, Node};
use crate::storage::allocated::{self, Allocated, LazyAllocated};
use crate::storage::error::Error;
use crate::storage::{Offset, Storage};
use alloc::collections::btree_map::Entry::{Occupied, Vacant};
use alloc::collections::{BTreeMap, BTreeSet};
use core::iter::IntoIterator;
use core::marker::PhantomData;
use core::result::Result;

/// The instance of Storage Linked List
pub struct StorageLinkedList<'a, I: Sized> {
    storage: &'a Storage,
    header: LazyAllocated<Header>,
    nodes: BTreeMap<Offset, Allocated<Node<I>>>,
}

impl<'a, I: Sized> StorageLinkedList<'a, I> {
    /// creates a new instance of Storage Linked List.
    pub fn create(storage: &'a Storage) -> Result<Allocated<Self>, Error> {
        let header = Header::new();
        let allocated_header = storage.allocate(header)?;

        Ok(Allocated::new(
            allocated_header.offset(),
            StorageLinkedList {
                storage,
                header: allocated_header.into(),
                nodes: BTreeMap::new(),
            },
            true,
        ))
    }

    /// load the Storage Linked List
    pub fn lazy_load(storage: &'a Storage, offset: Offset) -> Result<Allocated<Self>, Error> {
        Ok(Allocated::new(
            offset,
            StorageLinkedList {
                storage,
                header: offset.into(),
                nodes: BTreeMap::new(),
            },
            false,
        ))
    }

    /// pushes an item at the end of linked list.
    pub fn push_back(&mut self, item: I) -> Result<(), Error> {
        let allocated = self.storage.allocate(Node::new(item))?;
        let header = self.header.get_mut(self.storage)?.data_mut();
        let prev_tail_offset = header.tail_offset;
        header.count += 1;
        header.tail_offset = allocated.offset();

        if header.count == 1 {
            header.head_offset = allocated.offset();
        } else {
            let tail: &mut Allocated<Node<I>> = self.get_node(prev_tail_offset)?;
            tail.data_mut().next = allocated.offset();
        }

        self.nodes.insert(allocated.offset(), allocated);

        Ok(())
    }

    fn get_node(&mut self, offset: Offset) -> Result<&mut Allocated<Node<I>>, Error> {
        match self.nodes.entry(offset) {
            Occupied(o) => Ok(o.into_mut()),
            Vacant(v) => {
                let allocated_node = self.storage.read_struct(offset)?;
                Ok(v.insert(allocated_node))
            }
        }
    }
}

// pub struct StorageLinkedListIter<'a, I>{
//     list: &'a mut StorageLinkedList<'a, I>,
//     cur: Option<Allocated<Node<I>>>,
// }

// impl<'a, I> Iterator for StorageLinkedListIter<'a, I> {
//     type Item = &'a Allocated<Node<I>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match &self.cur {
//             Some(allocated) => {
//                 if allocated.data().next == 0 {
//                     None
//                 } else {
//                     Some(self.list.get_node(allocated.data().next).unwrap()) // TODO???
//                 }

//             }
//             None => None
//         }
//     }
// }

// impl<'a, I> IntoIterator for &'a mut StorageLinkedList<'a, I> {
//     type Item = &'a Allocated<Node<I>>;
//     type IntoIter = StorageLinkedListIter<'a, I>;

//     fn into_iter(self) -> Self::IntoIter {
//         StorageLinkedListIter{
//             list: self,
//             cur: None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::storage::mock::mock_storage;

    use super::StorageLinkedList;

    #[test]
    fn test_push_back() {
        let storage = mock_storage(4*1024);
        let mut linked_list = StorageLinkedList::<i32>::create(&storage).unwrap();
        linked_list.data_mut().push_back(1).unwrap();
    }
}
