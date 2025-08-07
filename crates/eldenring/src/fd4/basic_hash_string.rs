use std::{ffi, fmt::Display};

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
    /// Hashed representation of the string field.
    pub hash: u32,
    /// Indicates whether or not the hash field is populated.
    pub needs_hashing: bool,
    // _pad3d: [u8; 0x3],
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

#[cfg(test)]
mod test {
    use crate::fd4::FD4BasicHashString;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x40, size_of::<FD4BasicHashString>());
    }
}
