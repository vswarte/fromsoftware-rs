use std::error::Error;
use std::fmt::Display;
use std::mem;
use std::ptr::NonNull;

use crate::dlkr::{DLAllocatorBase, DLAllocatorVmt};

use encoding_rs;

use shared::OwnedPtr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DLCharacterSet {
    UTF8 = 0,
    #[default]
    UTF16 = 1,
    Iso8859_1 = 2,
    ShiftJis = 3,
    EucJp = 4,
    UTF32 = 5,
}

trait StringKindImpl {}
#[allow(private_bounds)]
pub trait StringKind: StringKindImpl {
    // type of the single character, eg u8, u16, etc
    type CharType: Sized + Copy + Default;
    // Enum variant for the character set
    const CHARSET: DLCharacterSet;
    // Default capacity in characters, without null terminator
    const DEFAULT_CAPACITY: usize;
    // Encoding used for this character set
    const ENCODING: &'static encoding_rs::Encoding;
    // Size and type of the inline buffer
    type InlineType: Sized + Copy + Default;

    fn to_utf8(buf: &[u8]) -> String {
        let (decoded, _, _) = Self::ENCODING.decode(buf);
        decoded.to_string()
    }

    fn from_utf8(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let (encoded, _, _) = Self::ENCODING.encode(s);
        Ok(encoded.into_owned())
    }
}

pub struct DLUTF8StringKind;
pub struct DLUTF16StringKind;
pub struct DLISO8859_1StringKind;
pub struct DLShiftJisStringKind;
pub struct DLEucJpStringKind;
// UTF-32 is not used in the game, and no standard encoding exists for it

impl StringKindImpl for DLUTF8StringKind {}
impl StringKind for DLUTF8StringKind {
    type CharType = u8;
    const CHARSET: DLCharacterSet = DLCharacterSet::UTF8;
    const DEFAULT_CAPACITY: usize = 15;
    const ENCODING: &'static encoding_rs::Encoding = encoding_rs::UTF_8;
    type InlineType = [u8; 16];
}
impl StringKindImpl for DLUTF16StringKind {}
impl StringKind for DLUTF16StringKind {
    type CharType = u16;
    const CHARSET: DLCharacterSet = DLCharacterSet::UTF16;
    const DEFAULT_CAPACITY: usize = 7;
    const ENCODING: &'static encoding_rs::Encoding = encoding_rs::UTF_16LE;
    type InlineType = [u16; 8];

    /// encoding-rs does not support UTF-16 encoding, so we have to do it manually
    fn from_utf8(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(s.encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>())
    }
}
impl StringKindImpl for DLISO8859_1StringKind {}
impl StringKind for DLISO8859_1StringKind {
    type CharType = u8;
    const CHARSET: DLCharacterSet = DLCharacterSet::Iso8859_1;
    const DEFAULT_CAPACITY: usize = 15;
    // ISO-8859-1 aka Latin-1 is exactly the same as Windows-1252
    const ENCODING: &'static encoding_rs::Encoding = encoding_rs::WINDOWS_1252;
    type InlineType = [u8; 16];
}
impl StringKindImpl for DLShiftJisStringKind {}
impl StringKind for DLShiftJisStringKind {
    type CharType = u8;
    const CHARSET: DLCharacterSet = DLCharacterSet::ShiftJis;
    const DEFAULT_CAPACITY: usize = 15;
    const ENCODING: &'static encoding_rs::Encoding = encoding_rs::SHIFT_JIS;
    type InlineType = [u8; 16];
}
impl StringKindImpl for DLEucJpStringKind {}
impl StringKind for DLEucJpStringKind {
    type CharType = u16;
    const CHARSET: DLCharacterSet = DLCharacterSet::EucJp;
    const DEFAULT_CAPACITY: usize = 7;
    const ENCODING: &'static encoding_rs::Encoding = encoding_rs::EUC_JP;
    type InlineType = [u16; 8];
}

#[repr(C)]
#[derive(Clone, Copy)]
union StringUnion<T: StringKind> {
    inline: T::InlineType,
    pointer: Option<NonNull<T::CharType>>,
}

impl<T: StringKind> Default for StringUnion<T> {
    fn default() -> Self {
        Self {
            inline: T::InlineType::default(),
        }
    }
}

#[repr(C)]
struct BasicString<T: StringKind> {
    inner: StringUnion<T>,
    length: usize,
    capacity: usize,
}

impl<T: StringKind> Default for BasicString<T> {
    fn default() -> Self {
        Self {
            inner: StringUnion::default(),
            length: 0,
            capacity: T::DEFAULT_CAPACITY,
        }
    }
}

#[repr(C)]
pub struct DLString<T: StringKind = DLUTF16StringKind> {
    allocator: NonNull<DLAllocatorBase>,
    base: BasicString<T>,
    encoding: DLCharacterSet,
}

impl<T: StringKind> DLString<T> {
    pub fn new<U: DLAllocatorVmt>(allocator: &U) -> Self {
        Self {
            allocator: NonNull::new(allocator as *const U as *mut DLAllocatorBase).unwrap(),
            base: BasicString::<T>::default(),
            encoding: T::CHARSET,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.base.length == 0
    }

    pub fn is_inlined(&self) -> bool {
        self.base.capacity == T::DEFAULT_CAPACITY
    }

    pub fn copy<U: DLAllocatorVmt>(
        allocator: &U,
        other: &DLString<T>,
    ) -> Result<Self, &'static str> {
        if other.is_empty() {
            return Ok(Self::new(allocator));
        }
        let mut string = Self::new(allocator);

        string.base.length = other.base.length;

        if other.is_inlined() {
            unsafe {
                let slice = other.get_inline_data();
                string.copy_inlined(slice, other.base.length)?
            };
        } else {
            string.base.capacity = other.base.capacity;
            let source = other.get_pointer_data();
            unsafe { string.copy_allocated(source, other.base.length)? };
        }

        Ok(string)
    }

    pub fn from_str<U: DLAllocatorVmt>(allocator: &U, s: &str) -> Result<Self, Box<dyn Error>> {
        if s.is_empty() {
            return Ok(Self::new(allocator));
        }

        let encoded = T::from_utf8(s)?;
        let char_count = encoded.len() / mem::size_of::<T::CharType>();
        let mut string = Self::new(allocator);

        unsafe {
            if char_count <= T::DEFAULT_CAPACITY {
                string.copy_inlined(&encoded, char_count)?;
            } else {
                string.copy_allocated(&encoded, char_count)?;
            }
        }

        string.base.length = char_count;
        Ok(string)
    }

    pub fn as_string(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let slice = if self.is_inlined() {
            self.get_inline_data()
        } else {
            self.get_pointer_data()
        };

        T::to_utf8(slice)
    }

    fn get_inline_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                &self.base.inner.inline as *const _ as *const u8,
                self.base.length * mem::size_of::<T::CharType>(),
            )
        }
    }

    fn get_pointer_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.base.inner.pointer.unwrap().as_ptr() as *const u8,
                self.base.length * mem::size_of::<T::CharType>(),
            )
        }
    }

    unsafe fn copy_inlined(&mut self, bytes: &[u8], length: usize) -> Result<(), &'static str> {
        if length > T::DEFAULT_CAPACITY {
            return Err("Length exceeds max inlined size");
        }

        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            &mut self.base.inner.inline as *mut _ as *mut u8,
            length * mem::size_of::<T::CharType>(),
        );
        self.null_terminate(length);
        self.base.length = length;
        Ok(())
    }

    unsafe fn copy_allocated(&mut self, source: &[u8], length: usize) -> Result<(), &'static str> {
        if length > self.base.capacity {
            self.deallocate();
            self.base.length = 0;
            self.reallocate(length)?;
        } else if self.base.inner.pointer.is_none() {
            let allocation = self.allocate(length + 1)?;
            self.base.inner.pointer = NonNull::new(allocation);
        }

        let dest_ptr = self.base.inner.pointer.unwrap().as_ptr();
        let byte_length = length * mem::size_of::<T::CharType>();

        std::ptr::copy_nonoverlapping(source.as_ptr(), dest_ptr as *mut u8, byte_length);

        self.base.length = length;
        self.null_terminate(length);
        Ok(())
    }

    unsafe fn null_terminate(&mut self, length: usize) {
        if self.is_inlined() {
            let ptr = &mut self.base.inner.inline as *mut T::InlineType as *mut T::CharType;
            *ptr.add(length) = T::CharType::default();
        } else {
            let ptr = self.base.inner.pointer.unwrap().as_ptr();
            *ptr.add(length) = T::CharType::default();
        }
    }

    unsafe fn allocate(&mut self, size: usize) -> Result<*mut T::CharType, &'static str> {
        let allocator = self.allocator.as_mut();
        let allocated = (allocator.vftable.allocate_aligned)(
            allocator,
            size * mem::size_of::<T::CharType>(),
            mem::align_of::<T::CharType>(),
        ) as *mut T::CharType;
        if allocated.is_null() {
            return Err("Allocation failed");
        }
        Ok(allocated)
    }

    unsafe fn deallocate(&mut self) {
        if !self.is_inlined() {
            if let Some(ptr) = self.base.inner.pointer {
                let allocator = self.allocator.as_mut();
                (allocator.vftable.deallocate)(allocator, ptr.as_ptr() as *mut u8);
            }
        }
    }

    unsafe fn reallocate(&mut self, new_length: usize) -> Result<(), &'static str> {
        if new_length <= self.base.capacity {
            return Ok(());
        }

        let allocation = self.allocate(new_length + 1)?;

        if !self.is_empty() {
            let src_slice = if self.is_inlined() {
                self.get_inline_data()
            } else {
                self.get_pointer_data()
            };

            let byte_length = self.base.length * mem::size_of::<T::CharType>();
            std::ptr::copy_nonoverlapping(src_slice.as_ptr(), allocation as *mut u8, byte_length);

            self.deallocate();
        }
        self.base.inner.pointer = NonNull::new(allocation);
        self.base.capacity = new_length;

        Ok(())
    }
}

impl<T: StringKind> Drop for DLString<T> {
    fn drop(&mut self) {
        unsafe { self.deallocate() };
    }
}

impl<T: StringKind> Display for DLString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[repr(C)]
pub struct DLRawString<T: StringKind = DLUTF16StringKind> {
    vftable: usize,
    pub backing_string: Option<NonNull<T::CharType>>,
    pub length: usize,
    unk18: u32,
    pub char_size: u16,
    pub encoding: DLCharacterSet,
    pub flags: u8,
}

impl<T: StringKind> DLRawString<T> {
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn as_string(&self) -> String {
        if self.is_empty() || self.backing_string.is_none() {
            return String::new();
        }

        T::to_utf8(unsafe {
            std::slice::from_raw_parts(
                self.backing_string.unwrap().as_ptr() as *const u8,
                self.length * mem::size_of::<T::CharType>(),
            )
        })
    }
}

impl<T: StringKind> Display for DLRawString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
pub type DLCodedString<T> = DLRawString<T>;

#[repr(C)]
/// Source of name: RTTI
/// In original code, generic type was something like `DLInplaceStr<1,16,DLTX::DLCodedStr<1>>`
/// Where 1 is DLCharacterSet::UTF16 and 16 is the size of the inline buffer in characters.
/// This version uses StringKind trait to get the character set and size.
pub struct DLInplaceStr<T: StringKind, const N: usize> {
    /// Underlying DLCodedString used for all operations.
    pub base: DLCodedString<T>,
    /// Buffer used to store the string data.
    /// backing_string in base is a pointer to this buffer.
    pub bytes: [T::CharType; N],
    unk: usize,
}

impl<T: StringKind, const N: usize> Display for DLInplaceStr<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base.as_string())
    }
}

#[cfg(test)]
mod tests {
    use vtable_rs::VPtr;

    use super::*;
    use crate::dlkr::{DLAllocatorBase, DLAllocatorVmt};
    use std::ptr::NonNull;

    #[derive(Default)]
    struct RustDLAllocator {
        vftable: VPtr<dyn DLAllocatorVmt, Self>,
    }
    impl DLAllocatorVmt for RustDLAllocator {
        extern "C" fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8 {
            let ptr = unsafe {
                std::alloc::alloc(std::alloc::Layout::from_size_align(size, alignment).unwrap())
            };
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr
        }
        extern "C" fn deallocate(&mut self, allocation: *const u8) {
            unsafe { std::alloc::dealloc(allocation as *mut u8, std::alloc::Layout::new::<u8>()) }
        }
        extern "C" fn allocate(&mut self, size: usize) -> *const u8 {
            self.allocate_aligned(size, mem::align_of::<u8>())
        }
        extern "C" fn destructor(&mut self, param_2: bool) {
            todo!()
        }
        extern "C" fn allocator_id(&self) -> u32 {
            todo!()
        }
        extern "C" fn unk10(&self) {
            todo!()
        }
        extern "C" fn heap_flags(&self) -> &u64 {
            todo!()
        }
        extern "C" fn heap_capacity(&self) -> usize {
            todo!()
        }
        extern "C" fn heap_size(&self) -> usize {
            todo!()
        }
        extern "C" fn backing_heap_capacity(&self) -> usize {
            todo!()
        }
        extern "C" fn heap_allocation_count(&self) -> usize {
            todo!()
        }
        extern "C" fn allocation_size(&self, allocation: *const u8) -> usize {
            todo!()
        }
        extern "C" fn reallocate(&mut self, allocation: *const u8, size: usize) -> *const u8 {
            todo!()
        }
        extern "C" fn reallocate_aligned(
            &mut self,
            allocation: *const u8,
            size: usize,
            alignment: usize,
        ) -> *const u8 {
            todo!()
        }
        extern "C" fn allocate_second(&mut self, size: usize) -> *const u8 {
            todo!()
        }
        extern "C" fn allocate_aligned_second(
            &mut self,
            size: usize,
            alignment: usize,
        ) -> *const u8 {
            todo!()
        }
        extern "C" fn reallocate_second(
            &mut self,
            allocation: *const u8,
            size: usize,
        ) -> *const u8 {
            todo!()
        }
        extern "C" fn reallocate_aligned_second(
            &mut self,
            allocation: *const u8,
            size: usize,
            alignment: usize,
        ) -> *const u8 {
            todo!()
        }
        extern "C" fn deallocate_second(&mut self, allocation: *const u8) {
            todo!()
        }
        extern "C" fn unka0(&self) -> bool {
            todo!()
        }
        extern "C" fn allocation_belongs_to_first_allocator(
            &mut self,
            allocation: *const u8,
        ) -> bool {
            todo!()
        }
        extern "C" fn allocation_belongs_to_second_allocator(
            &mut self,
            allocation: *const u8,
        ) -> bool {
            todo!()
        }
        extern "C" fn lock(&mut self) {
            todo!()
        }
        extern "C" fn unlock(&mut self) {
            todo!()
        }
        extern "C" fn get_memory_block_for_allocation(
            &mut self,
            allocation: *const u8,
        ) -> *const u8 {
            todo!()
        }
    }

    #[test]
    fn test_dlstring_encodings() {
        let allocator = RustDLAllocator::default();
        let dl_string = DLString::<DLUTF8StringKind>::new(&allocator);
        assert_eq!(dl_string.encoding, DLCharacterSet::UTF8);
        assert_eq!(dl_string.base.capacity, 15);

        let dl_string = DLString::<DLUTF16StringKind>::new(&allocator);
        assert_eq!(dl_string.encoding, DLCharacterSet::UTF16);
        assert_eq!(dl_string.base.capacity, 7);
        assert!(dl_string.is_empty());

        let utf8_string = DLString::<DLUTF8StringKind>::from_str(&allocator, "Hello").unwrap();
        assert_eq!(utf8_string.encoding, DLCharacterSet::UTF8);
        assert_eq!(utf8_string.as_string(), "Hello");
        assert_eq!(utf8_string.base.length, 5);

        let latin1_string =
            DLString::<DLISO8859_1StringKind>::from_str(&allocator, "Hello").unwrap();
        assert_eq!(latin1_string.encoding, DLCharacterSet::Iso8859_1);
        assert_eq!(latin1_string.as_string(), "Hello");
        assert_eq!(latin1_string.base.length, 5);

        let test_str = "こんにちは";
        let shift_jis = DLString::<DLShiftJisStringKind>::from_str(&allocator, test_str).unwrap();
        assert_eq!(shift_jis.encoding, DLCharacterSet::ShiftJis);
        assert_eq!(shift_jis.as_string(), test_str);
    }

    #[test]
    fn test_dlstring_copy() {
        let allocator = RustDLAllocator::default();
        let dl_string1 = DLString::<DLUTF16StringKind>::from_str(&allocator, "Hello").unwrap();
        let dl_string2 = DLString::copy(&allocator, &dl_string1).unwrap();

        assert_eq!(dl_string1.as_string(), dl_string2.as_string());
        assert_eq!(dl_string1.base.length, dl_string2.base.length);
        assert_eq!(dl_string1.base.capacity, dl_string2.base.capacity);

        assert!(dl_string1.is_inlined());
        assert!(dl_string2.is_inlined());

        let long_string = "This is a longer string that definitely exceeds the inline capacity of 7 UTF-16 characters";
        let dl_long1 = DLString::<DLUTF16StringKind>::from_str(&allocator, long_string).unwrap();
        let dl_long2 = DLString::copy(&allocator, &dl_long1).unwrap();

        assert_eq!(dl_long1.as_string(), dl_long2.as_string());
        assert_eq!(dl_long1.base.length, dl_long2.base.length);
        assert_eq!(dl_long1.base.capacity, dl_long2.base.capacity);

        assert!(!dl_long1.is_inlined());
        assert!(!dl_long2.is_inlined());

        assert_ne!(
            unsafe { dl_long1.base.inner.pointer.unwrap().as_ptr() },
            unsafe { dl_long2.base.inner.pointer.unwrap().as_ptr() }
        );

        let empty = DLString::<DLUTF16StringKind>::new(&allocator);
        let copy = DLString::copy(&allocator, &empty).unwrap();

        assert!(copy.is_empty());
        assert_eq!(copy.base.length, 0);
        assert_eq!(copy.base.capacity, DLUTF16StringKind::DEFAULT_CAPACITY);
    }

    #[test]
    fn test_dlstring_empty() {
        let allocator = RustDLAllocator::default();
        let dl_string: DLString = DLString::from_str(&allocator, "").unwrap();

        assert!(dl_string.is_empty());
        assert_eq!(dl_string.base.length, 0);
        assert_eq!(dl_string.as_string(), "");
    }

    #[test]
    fn test_dlstring_exact_capacity_boundary() {
        let allocator = RustDLAllocator::default();

        let at_capacity = DLString::<DLUTF16StringKind>::from_str(&allocator, "1234567").unwrap();
        assert_eq!(at_capacity.base.length, 7);
        assert!(at_capacity.is_inlined());
        assert_eq!(at_capacity.as_string(), "1234567");

        let over_capacity =
            DLString::<DLUTF16StringKind>::from_str(&allocator, "12345678").unwrap();
        assert_eq!(over_capacity.base.length, 8);
        assert!(!over_capacity.is_inlined());
        assert_eq!(over_capacity.as_string(), "12345678");
    }

    #[test]
    fn test_dlstring_display() {
        let allocator = RustDLAllocator::default();
        let dl_string =
            DLString::<DLUTF16StringKind>::from_str(&allocator, "Display test").unwrap();

        let displayed = format!("{}", dl_string);
        assert_eq!(displayed, "Display test");
    }

    #[test]
    fn test_dlstring_long_string_operations() {
        let allocator = RustDLAllocator::default();

        let long_text =
            "This is a very long string that will definitely exceed the inline capacity.
            It's important to test how the string handles allocation, deallocation, and copying
            when dealing with long content that requires heap allocation.";

        let long_string = DLString::<DLUTF16StringKind>::from_str(&allocator, long_text).unwrap();

        assert!(!long_string.is_inlined());
        assert_eq!(long_string.as_string(), long_text);

        let long_copy = DLString::copy(&allocator, &long_string).unwrap();
        assert_eq!(long_copy.as_string(), long_text);
        assert!(!long_copy.is_inlined());
    }
}
