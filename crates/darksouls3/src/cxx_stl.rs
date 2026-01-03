use cxx_stl::vec::msvc2012 as vec;

use crate::dlkr::DLAllocatorRef;

/// An MSVC 2012-compatible vector that contains a custom DS3 allocator. This is
/// the type of vector generally used in DS3.
pub type CxxVec<T, A = DLAllocatorRef> = vec::CxxVec<T, A>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<CxxVec<usize>>());
    }
}
