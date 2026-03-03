use std::{cmp::PartialEq, fmt::Display};

use crate::dltx::{DLString, DLStringKind, DLUTF16StringKind};

#[repr(C)]
/// A string wrapper that caches the associated hash code.
///
/// This is frequently used in the resource system, which is built on hash maps.
/// It's occasioanlly used elsewhere as well.
///
/// Source of name: RTTI
pub struct FD4BasicHashString<T: DLStringKind = DLUTF16StringKind> {
    vftable: usize,

    /// The inner string.
    pub inner: DLString<T>,

    /// The string's hash code, or 0 if it hasn't yet been computed.
    pub hash: u32,

    /// Whether or not [Self::hash] has been computed.
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

impl<T: DLStringKind, S: AsRef<str>> PartialEq<S> for FD4BasicHashString<T> {
    fn eq(&self, other: &S) -> bool {
        self.inner.eq(other)
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
