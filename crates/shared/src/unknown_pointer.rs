use std::{ffi::c_void, fmt};

use crate::{Program, vftable_classname};

/// A pointer whose target has not yet been reverse-engineered.
///
/// ## Safety
///
/// This may be null and has no particular alignment requirements, but it does
/// require that if it's not nullit must point within the current process's
/// memory space.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UnknownPtr(*const c_void);

impl UnknownPtr {
    /// Interprets `value` as a pointer.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `value` is either 0 or a valid address in
    /// the current address space.
    pub unsafe fn from(value: usize) -> Self {
        UnknownPtr(value as *const c_void)
    }

    /// Returns the class name for this pointer, if it's pointing to a vftable
    /// whose name is in the RTTI.
    pub fn rtti_classname(&self) -> Option<String> {
        if self.0.is_null() {
            return None;
        }

        let usize_ptr = self.cast::<usize>();
        if !usize_ptr.is_aligned() {
            return None;
        }

        // Even though we don't have any guarantees about what the pointer
        // points to, we know it's aligned for `usize` and it's within the
        // current address space so dereferencing it should be safe.
        vftable_classname(&Program::current(), unsafe { *usize_ptr })
    }

    /// Returns `true` if this pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Casts this to a pointer to `T`.
    pub fn cast<T>(self) -> *const T {
        self.0.cast::<T>()
    }

    /// Gets the "address" portion of the pointer.
    pub fn addr(self) -> usize {
        self.0.addr()
    }
}

impl fmt::Debug for UnknownPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(name) = self.rtti_classname() {
            write!(f, "{:?} [{name}*]", self.0)
        } else {
            fmt::Debug::fmt(&self.0, f)
        }
    }
}

impl fmt::Pointer for UnknownPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(name) = self.rtti_classname() {
            write!(f, "{:p} [{name}*]", self.0)
        } else {
            fmt::Pointer::fmt(&self.0, f)
        }
    }
}
