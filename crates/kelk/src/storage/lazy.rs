use super::allocated::Allocated;
use super::error::Error;
use super::{Offset, Storage};

pub enum LazyAllocated<T> {
    Offset(Offset),
    Allocated(Allocated<T>),
}

impl<T> LazyAllocated<T> {
    fn read(&mut self, storage: &Storage) -> Result<(), Error> {
        if let Lazy::Index(index) = self {
            let offset = storage.read_stack_at(*index)?;
            *self = Lazy::Object(offset, storage.read_struct(offset)?);
        }
        Ok(())
    }

    pub(crate) fn get(&mut self, storage: &Storage) -> Result<&mut T, Error> {
        self.read(storage)?;
        if let Lazy::Object(_, obj) = self {
            Ok(obj)
        } else {
            unreachable!()
        }
    }

    pub(crate) fn offset(&mut self, storage: &Storage) -> Result<Offset, Error> {
        self.read(storage)?;
        if let Lazy::Object(offset, _) = self {
            Ok(*offset)
        } else {
            unreachable!()
        }
    }
}

impl<T> From<Allocated<T>> for Lazy<T> {
    fn from(allocated: Allocated<T>) -> Self {
        Lazy::Object(allocated.offset(), allocated.object_move())
    }
}

impl<T> From<u16> for Lazy<T> {
    fn from(stack_index: u16) -> Self {
        Lazy::Index(stack_index)
    }
}
