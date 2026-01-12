use std::{fmt, ops::Deref, ptr::NonNull};

/// Pointer to a structure that is safe to consume as long as the safety rules of the
/// containing type is upheld.
///
/// This type should not be used outside of types that mirror a type from the game.
///
/// # Safety
///
/// Consumer must ensure:
///  - that the safety rules of the containing type are upheld.
#[repr(transparent)]
pub struct ReadOnlyPtr<T>(NonNull<T>);

impl<T> ReadOnlyPtr<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Deref for ReadOnlyPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsRef<T> for ReadOnlyPtr<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> fmt::Debug for ReadOnlyPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

unsafe impl<T> Send for ReadOnlyPtr<T> {}
unsafe impl<T> Sync for ReadOnlyPtr<T> where T: Sync {}
