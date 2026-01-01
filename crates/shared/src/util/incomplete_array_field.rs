use std::fmt;
use std::marker::PhantomData;
use std::slice;

/// This is an adaptation of the __IncompleteArrayField type that rust-bindgen
/// generates.
#[repr(transparent)]
#[derive(Default)]
pub struct IncompleteArrayField<T>(PhantomData<T>, [T; 0]);
impl<T> IncompleteArrayField<T> {
    /// Creates a new field with no contents.
    #[inline]
    pub const fn new() -> Self {
        IncompleteArrayField(PhantomData, [])
    }

    /// Returns a constant pointer to the beginning of this field.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    /// Returns a mutable pointer to the beginning of this field.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    /// Returns a slice representing the data in this field.
    ///
    /// ## Safety
    ///
    /// The caller must have out-of-band knowledge that the field is at least
    /// [len] entries long.
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), len) }
    }

    /// Returns a mutable slice representing the data in this field.
    ///
    /// ## Safety
    ///
    /// The caller must have out-of-band knowledge that the field is at least
    /// [len] entries long.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}

impl<T> ::std::fmt::Debug for IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
