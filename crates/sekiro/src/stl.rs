use std::ptr::NonNull;

use crate::dlkr::DLAllocatorBase;

#[repr(C)]
pub struct BasicVector<T>
where
    T: Sized,
{
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<T> BasicVector<T>
where
    T: Sized,
{
    pub fn items(&self) -> &[T] {
        let Some(start) = self.begin else {
            return &mut [];
        };

        let end = self.end.unwrap();
        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        unsafe { std::slice::from_raw_parts(start.as_ptr(), count) }
    }

    pub fn items_mut(&mut self) -> &mut [T] {
        let Some(start) = self.begin else {
            return &mut [];
        };

        let end = self.end.unwrap();
        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        unsafe { std::slice::from_raw_parts_mut(start.as_ptr(), count) }
    }

    pub fn len(&self) -> usize {
        let Some(end) = self.end else {
            return 0;
        };

        let Some(start) = self.begin else {
            return 0;
        };

        (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[repr(C)]
pub struct Vector<T>
where
    T: Sized,
{
    allocator: NonNull<DLAllocatorBase>,
    pub base: BasicVector<T>,
}

impl<T> Vector<T> {
    pub fn items(&self) -> &[T] {
        self.base.items()
    }

    pub fn items_mut(&mut self) -> &mut [T] {
        self.base.items_mut()
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }
}
