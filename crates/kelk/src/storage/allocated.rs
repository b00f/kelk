//!
//!
use super::error::Error;
use super::Offset;
use super::Storage;


///
pub struct Allocated<T: Sized> {
    offset: Offset,
    data: T,
    invalidated: bool,
}

///
pub enum LazyAllocated<T: Sized> {
    ///
    Offset(Offset),
    ///
    Allocated(Allocated<T>),
}

impl<T: Sized> LazyAllocated<T> {
    fn read(&mut self, storage: &Storage) -> Result<(), Error> {
        if let LazyAllocated::Offset(offset) = self {
            *self = LazyAllocated::Allocated(storage.read_struct(*offset)?);
        }
        Ok(())
    }

    pub(crate) fn get(&mut self, storage: &Storage) -> Result<&Allocated<T>, Error> {
        self.read(storage)?;
        if let LazyAllocated::Allocated(allocated) = self {
            Ok(allocated)
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_mut(&mut self, storage: &Storage) -> Result<&mut Allocated<T>, Error> {
        self.read(storage)?;
        if let LazyAllocated::Allocated(allocated) = self {
            Ok(allocated)
        } else {
            unreachable!()
        }
    }

    // pub(crate) fn offset(&mut self, storage: &Storage) -> Result<Offset, Error> {
    //     self.read(storage)?;
    //     if let Lazy::Object(offset, _) = self {
    //         Ok(*offset)
    //     } else {
    //         unreachable!()
    //     }
    // }
}

impl<T: Sized> Allocated<T> {
    ///
    pub fn new(offset: Offset, data: T, invalidated: bool) -> Self {
        Allocated {
            offset,
            data,
            invalidated,
        }
    }
    ///
    pub fn offset(&self) -> Offset {
        self.offset
    }

    ///
    pub fn data(&self) -> &T {
        &self.data
    }

    ///
    pub fn data_mut(&mut self) -> &mut T {
        self.invalidated = true;
        &mut self.data
    }

    ///
    pub fn write(&self, storage: &Storage) -> Result<(), Error> {
        if self.invalidated {
            storage.write_struct(self.offset, &self.data)?;
        }

        Ok(())
    }
}

impl<T: Sized> From<Allocated<T>> for LazyAllocated<T> {
    fn from(allocated: Allocated<T>) -> Self {
        LazyAllocated::Allocated(allocated)
    }
}

impl<T: Sized> From<Offset> for LazyAllocated<T> {
    fn from(offset: Offset) -> Self {
        LazyAllocated::Offset(offset)
    }
}
