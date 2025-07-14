use std::alloc::Layout;
use std::borrow::Cow;
use std::fmt::Display;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{alloc::GlobalAlloc, error::Error};

use crate::dlkr::{DLAllocatorBase, DLAllocatorVmt};

use encoding_rs;

use thiserror::Error;

use cxx_stl::string::{
    CxxNarrowString, CxxUtf16String, CxxUtf32String, CxxUtf8String, CxxWideString,
};

use shared::OwnedPtr;

#[repr(transparent)]
#[derive(Clone)]
pub struct DLAllocatorRef(NonNull<DLAllocatorBase>);

unsafe impl GlobalAlloc for DLAllocatorRef {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = self.0.as_ptr();
        ((*allocator).vftable.allocate)(&mut *allocator, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let allocator = self.0.as_ptr();
        ((*allocator).vftable.deallocate)(&mut *allocator, ptr);
    }
}

impl From<NonNull<DLAllocatorBase>> for DLAllocatorRef {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

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

#[derive(Error, Debug)]
pub enum DLStringEncodingError {
    #[error("Invalid encoding; expected {expected:?} but got {actual}")]
    InvalidEncoding {
        expected: DLCharacterSet,
        actual: u8,
    },
    #[error("Error decoding string")]
    DecodeError,
    #[error("Error encoding string")]
    EncodeError,
    #[error("Unsupported encoding: {0}")]
    UnsupportedEncoding(u8),
}

trait CxxString<CharType> {
    fn new_in(allocator: DLAllocatorRef) -> Self;
    fn from_bytes_in(bytes: &[CharType], allocator: DLAllocatorRef) -> Self;
    fn as_u8_slice(&self) -> &[u8];
    fn as_bytes(&self) -> &[CharType];
}

macro_rules! impl_cxx_string {
    ($string_type:ty, $char_type:ty) => {
        impl CxxString<$char_type> for $string_type {
            fn new_in(allocator: DLAllocatorRef) -> Self {
                Self::new_in(allocator)
            }

            fn from_bytes_in(bytes: &[$char_type], allocator: DLAllocatorRef) -> Self {
                Self::from_bytes_in(bytes, allocator)
            }
            fn as_u8_slice(&self) -> &[u8] {
                unsafe {
                    std::slice::from_raw_parts(
                        self.as_ptr() as *const u8,
                        self.len() * std::mem::size_of::<$char_type>(),
                    )
                }
            }
            fn as_bytes(&self) -> &[$char_type] {
                self.as_bytes()
            }
        }
    };
}

impl_cxx_string!(CxxUtf16String<DLAllocatorRef>, u16);
impl_cxx_string!(CxxUtf8String<DLAllocatorRef>, u8);
impl_cxx_string!(CxxNarrowString<DLAllocatorRef>, u8);
impl_cxx_string!(CxxUtf32String<DLAllocatorRef>, u32);

trait DLStringKindSeal {}

/// This trait is used to seal the DLStringKind trait, preventing external implementations.
#[allow(private_bounds)]
pub trait DLStringKind: DLStringKindSeal {
    type InnerType: CxxString<Self::CharType>;
    type CharType: Sized + Copy;
    const ENCODING: DLCharacterSet;

    fn encode(s: &str) -> Result<Vec<Self::CharType>, DLStringEncodingError> {
        match Self::ENCODING {
            DLCharacterSet::UTF16 => {
                let mut bytes: Vec<u16> = Vec::with_capacity(s.len() * size_of::<u16>());
                bytes.extend(s.encode_utf16());
                // SAFETY: Transmuting Vec<u16> to Vec<Self::CharType> is safe
                // because the UTF16 arm ensures CharType is u16.
                Ok(unsafe { std::mem::transmute::<Vec<u16>, Vec<Self::CharType>>(bytes) })
            }
            DLCharacterSet::UTF32 => {
                let mut bytes: Vec<u32> = Vec::with_capacity(s.len() * size_of::<u32>());
                bytes.extend(s.chars().map(|c| c as u32));
                // SAFETY: Transmuting Vec<u32> to Vec<Self::CharType> is safe
                // because the UTF32 arm ensures CharType is u32.
                Ok(unsafe { std::mem::transmute::<Vec<u32>, Vec<Self::CharType>>(bytes) })
            }
            DLCharacterSet::UTF8 => {
                /// We can just slice the string as UTF-8 bytes because Rust's `str` is UTF-8 encoded.
                let bytes = s.as_bytes().to_vec();
                // SAFETY: Transmuting Vec<u8> to Vec<Self::CharType> is safe
                // because this arm ensures CharType is u8.
                Ok(unsafe { std::mem::transmute::<Vec<u8>, Vec<Self::CharType>>(bytes) })
            }
            _ => {
                let encoding = match Self::ENCODING {
                    DLCharacterSet::Iso8859_1 => encoding_rs::WINDOWS_1252,
                    DLCharacterSet::ShiftJis => encoding_rs::SHIFT_JIS,
                    DLCharacterSet::EucJp => encoding_rs::EUC_JP,
                    _ => unreachable!(),
                };
                let (encoded_bytes, _, had_errors) = encoding.encode(s);
                if had_errors {
                    return Err(DLStringEncodingError::EncodeError);
                }

                // SAFETY: Transmuting Vec<u8> to Vec<Self::CharType> is safe
                // because this arm ensures CharType is u8.
                Ok(unsafe {
                    std::mem::transmute::<Vec<u8>, Vec<Self::CharType>>(encoded_bytes.into_owned())
                })
            }
        }
    }

    fn decode(s: &[u8]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        match Self::ENCODING {
            DLCharacterSet::UTF16 => {
                if !s.len().is_multiple_of(std::mem::size_of::<u16>()) {
                    return Err(DLStringEncodingError::DecodeError);
                }
                let u16_slice =
                    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u16, s.len() / 2) };
                char::decode_utf16(u16_slice.iter().cloned())
                    .map(|r| r.map_err(|_| DLStringEncodingError::DecodeError))
                    .collect::<Result<String, _>>()
                    .map(Cow::Owned)
            }
            DLCharacterSet::UTF32 => {
                if !s.len().is_multiple_of(std::mem::size_of::<u32>()) {
                    return Err(DLStringEncodingError::DecodeError);
                }
                let u32_slice =
                    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u32, s.len() / 4) };
                u32_slice
                    .iter()
                    .map(|&c| std::char::from_u32(c).ok_or(DLStringEncodingError::DecodeError))
                    .collect::<Result<String, _>>()
                    .map(Cow::Owned)
            }
            DLCharacterSet::UTF8 => {
                let s = std::str::from_utf8(s).map_err(|_| DLStringEncodingError::DecodeError)?;
                Ok(Cow::Borrowed(s))
            }
            _ => {
                let encoding = match Self::ENCODING {
                    DLCharacterSet::Iso8859_1 => encoding_rs::WINDOWS_1252,
                    DLCharacterSet::ShiftJis => encoding_rs::SHIFT_JIS,
                    DLCharacterSet::EucJp => encoding_rs::EUC_JP,
                    _ => unreachable!(),
                };
                let (cow, _, had_errors) = encoding.decode(s);
                if had_errors {
                    Err(DLStringEncodingError::DecodeError)
                } else {
                    Ok(cow)
                }
            }
        }
    }
}

pub struct DLUTF8StringKind;
impl DLStringKindSeal for DLUTF8StringKind {}
impl DLStringKind for DLUTF8StringKind {
    type InnerType = CxxUtf8String<DLAllocatorRef>;
    type CharType = u8;
    const ENCODING: DLCharacterSet = DLCharacterSet::UTF8;
}

pub struct DLISO8859_1StringKind;
impl DLStringKindSeal for DLISO8859_1StringKind {}
impl DLStringKind for DLISO8859_1StringKind {
    type InnerType = CxxNarrowString<DLAllocatorRef>;
    type CharType = u8;
    const ENCODING: DLCharacterSet = DLCharacterSet::Iso8859_1;
}

pub struct DLShiftJisStringKind;
impl DLStringKindSeal for DLShiftJisStringKind {}
impl DLStringKind for DLShiftJisStringKind {
    type InnerType = CxxNarrowString<DLAllocatorRef>;
    type CharType = u8;
    const ENCODING: DLCharacterSet = DLCharacterSet::ShiftJis;
}

pub struct DLEucJpStringKind;
impl DLStringKindSeal for DLEucJpStringKind {}
impl DLStringKind for DLEucJpStringKind {
    type InnerType = CxxNarrowString<DLAllocatorRef>;
    type CharType = u8;
    const ENCODING: DLCharacterSet = DLCharacterSet::EucJp;
}

pub struct DLUTF16StringKind;
impl DLStringKindSeal for DLUTF16StringKind {}
impl DLStringKind for DLUTF16StringKind {
    type InnerType = CxxUtf16String<DLAllocatorRef>;
    type CharType = u16;
    const ENCODING: DLCharacterSet = DLCharacterSet::UTF16;
}

pub struct DLUTF32StringKind;
impl DLStringKindSeal for DLUTF32StringKind {}
impl DLStringKind for DLUTF32StringKind {
    type InnerType = CxxUtf32String<DLAllocatorRef>;
    type CharType = u32;
    const ENCODING: DLCharacterSet = DLCharacterSet::UTF32;
}

#[repr(C)]
pub struct DLString<T: DLStringKind = DLUTF16StringKind> {
    base: T::InnerType,
    encoding: DLCharacterSet,
}

impl<T: DLStringKind> DLString<T> {
    pub fn new(allocator: DLAllocatorRef) -> Self {
        Self {
            base: T::InnerType::new_in(allocator.clone()),
            encoding: T::ENCODING,
        }
    }

    pub fn from_str(allocator: DLAllocatorRef, s: &str) -> Result<Self, DLStringEncodingError> {
        let encoded: Vec<T::CharType> = T::encode(s)?;

        Ok(Self {
            base: T::InnerType::from_bytes_in(&encoded, allocator.clone()),
            encoding: T::ENCODING,
        })
    }

    pub fn to_str(&self) -> Result<String, DLStringEncodingError> {
        let bytes = self.base.as_u8_slice();
        T::decode(bytes).map(|cow| cow.into_owned())
    }

    pub fn copy<U: DLStringKind>(
        allocator: DLAllocatorRef,
        other: &DLString<U>,
    ) -> Result<Self, DLStringEncodingError> {
        // If the encodings match, we can directly copy the bytes
        if T::ENCODING == U::ENCODING {
            // SAFETY: T::ENCODING == U::ENCODING implies T::CharType is compatible with U::CharType.
            let bytes: &[T::CharType] = unsafe { std::mem::transmute(other.base.as_bytes()) };
            Ok(Self {
                base: T::InnerType::from_bytes_in(bytes, allocator.clone()),
                encoding: T::ENCODING,
            })
        } else {
            // If encodings differ, we need to decode and re-encode
            let decoded = T::decode(other.base.as_u8_slice())?;
            DLString::from_str(allocator, &decoded)
        }
    }
}

impl<T: DLStringKind> Deref for DLString<T> {
    type Target = T::InnerType;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: DLStringKind> DerefMut for DLString<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: DLStringKind> Display for DLString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_str() {
            Ok(s) => write!(f, "{s}"),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[repr(C)]
pub struct DLRawString<T: DLStringKind = DLUTF16StringKind> {
    vftable: usize,
    backing_string: Option<NonNull<T::CharType>>,
    pub length: usize,
    unk18: u32,
    pub char_size: u16,
    pub encoding: DLCharacterSet,
    pub flags: u8,
}

impl<T: DLStringKind> DLRawString<T> {
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn to_str(&self) -> Result<String, DLStringEncodingError> {
        let bytes: &[u8] = self.backing_string.as_ref().map_or(&[], |ptr| unsafe {
            std::slice::from_raw_parts(
                ptr.as_ptr() as *const u8,
                self.length * std::mem::size_of::<T::CharType>(),
            )
        });
        T::decode(bytes).map(|cow| cow.into_owned())
    }
}

impl<T: DLStringKind> Display for DLRawString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_str() {
            Ok(s) => write!(f, "{s}"),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
pub type DLCodedString<T> = DLRawString<T>;

#[repr(C)]
/// Source of name: RTTI
/// In original code, generic type was something like `DLInplaceStr<1,16,DLTX::DLCodedStr<1>>`
/// Where 1 is DLCharacterSet::UTF16 and 16 is the size of the inline buffer in characters.
/// This version uses StringKind trait to get the character set and size.
pub struct DLInplaceStr<T: DLStringKind, const N: usize> {
    /// Underlying DLCodedString used for all operations.
    pub base: DLCodedString<T>,
    /// Buffer used to store the string data.
    /// backing_string in base is a pointer to this buffer.
    pub bytes: [T::CharType; N],
    unk: usize,
}

impl<T: DLStringKind, const N: usize> Display for DLInplaceStr<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}
