use std::{
    fmt,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// Pointer to a structure that the containing structure owns. You will generally use this to model
/// structures in foreign memory when extending the game libraries. Do not use this in your own
/// code as you're risking all rusts safety reasoning.
///
/// # Safety
///
/// User must ensure that it's safe for this pointer to be turned into a (potentially mutable)
/// reference if a reference to its embedding structure is obtained.
#[repr(transparent)]
pub struct OwnedPtr<T>(NonNull<T>);

impl<T> OwnedPtr<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Deref for OwnedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsRef<T> for OwnedPtr<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> DerefMut for OwnedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> AsMut<T> for OwnedPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T> fmt::Debug for OwnedPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

unsafe impl<T> Send for OwnedPtr<T> {}
unsafe impl<T> Sync for OwnedPtr<T> where T: Sync {}
