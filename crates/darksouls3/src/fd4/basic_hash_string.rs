use std::fmt::Display;

use crate::dltx::{DLString, DLStringKind, DLUTF16StringKind};

#[repr(C)]
/// Wraps a string to make it easier to use with hashmaps. Seemingly mostly used in the resource
/// system but has some usage elsewhere too.
///
/// Source of name: RTTI
pub struct FD4BasicHashString<T: DLStringKind = DLUTF16StringKind> {
    vftable: usize,
    /// The contained string we're hashing for.
    pub inner: DLString<T>,
    // The rest of this is probably the same as in ER, but this hasn't been
    // verified yet.
}

impl<T: DLStringKind> AsRef<DLString<T>> for FD4BasicHashString<T> {
    fn as_ref(&self) -> &DLString<T> {
        &self.inner
    }
}

impl<T: DLStringKind> Display for FD4BasicHashString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
