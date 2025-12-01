use std::borrow::Cow;
use std::fmt::Display;

use crate::dlkr::DLAllocatorRef;

use encoding_rs;
use thiserror::Error;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum DLCharacterSet {
    UTF8 = 0,
    #[default]
    UTF16 = 1,
    Iso8859_1 = 2,
    ShiftJis = 3,
    EucJp = 4,
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

/// This trait is used to seal the DLStringKind trait, preventing external implementations.
trait DLStringKindSeal {}

#[allow(private_bounds)]
pub trait DLStringKind: DLStringKindSeal {
    type CharType: Sized + Copy;
    type InlineType: Copy + AsRef<[Self::CharType]>;

    fn decode(s: &[Self::CharType]) -> Result<Cow<'_, str>, DLStringEncodingError>;
}

pub struct DLUTF8StringKind;
impl DLStringKindSeal for DLUTF8StringKind {}
impl DLStringKind for DLUTF8StringKind {
    type CharType = u8;
    type InlineType = [u8; 16 / size_of::<u8>()];

    fn decode(s: &[u8]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        let s = std::str::from_utf8(s).map_err(|_| DLStringEncodingError::DecodeError)?;
        Ok(Cow::Borrowed(s))
    }
}

pub struct DLISO8859_1StringKind;
impl DLStringKindSeal for DLISO8859_1StringKind {}
impl DLStringKind for DLISO8859_1StringKind {
    type CharType = u8;
    type InlineType = [u8; 16 / size_of::<u8>()];

    fn decode(s: &[u8]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        let (cow, _, had_errors) = encoding_rs::WINDOWS_1252.decode(s);
        if had_errors {
            Err(DLStringEncodingError::DecodeError)
        } else {
            Ok(cow)
        }
    }
}

pub struct DLShiftJisStringKind;
impl DLStringKindSeal for DLShiftJisStringKind {}
impl DLStringKind for DLShiftJisStringKind {
    type CharType = u8;
    type InlineType = [u8; 16 / size_of::<u8>()];

    fn decode(s: &[u8]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        let (cow, _, had_errors) = encoding_rs::SHIFT_JIS.decode(s);
        if had_errors {
            Err(DLStringEncodingError::DecodeError)
        } else {
            Ok(cow)
        }
    }
}

pub struct DLEucJpStringKind;
impl DLStringKindSeal for DLEucJpStringKind {}
impl DLStringKind for DLEucJpStringKind {
    type CharType = u8;
    type InlineType = [u8; 16 / size_of::<u8>()];

    fn decode(s: &[u8]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        let (cow, _, had_errors) = encoding_rs::EUC_JP.decode(s);
        if had_errors {
            Err(DLStringEncodingError::DecodeError)
        } else {
            Ok(cow)
        }
    }
}

pub struct DLUTF16StringKind;
impl DLStringKindSeal for DLUTF16StringKind {}
impl DLStringKind for DLUTF16StringKind {
    type CharType = u16;
    type InlineType = [u16; 16 / size_of::<u16>()];

    fn decode(s: &[u16]) -> Result<Cow<'_, str>, DLStringEncodingError> {
        char::decode_utf16(s.iter().cloned())
            .map(|r| r.map_err(|_| DLStringEncodingError::DecodeError))
            .collect::<Result<String, _>>()
            .map(Cow::Owned)
    }
}

#[repr(C)]
union DLStringText<T: DLStringKind> {
    pointer: *const T::CharType,
    inline: T::InlineType,
}

#[repr(C)]
pub struct DLString<T: DLStringKind = DLUTF16StringKind> {
    text: DLStringText<T>,
    length: usize,
    capacity: usize,
    allocator: DLAllocatorRef,
    encoding: DLCharacterSet,
}

impl<T: DLStringKind> DLString<T> {
    pub fn to_str(&self) -> Result<String, DLStringEncodingError> {
        // Make sure this lives long enough to get assigned to `characters`.
        let inline: T::InlineType;
        // Strict < because C++ still adds a trailing null for C compat
        let characters = if self.length < size_of::<T::InlineType>() / size_of::<T::CharType>() {
            // SAFETY: We expect the original program to always store text
            // inline when it's safe to do so.
            inline = unsafe { self.text.inline };
            &inline.as_ref()[..self.length]
        } else {
            // SAFETY: We know that text is a pointer because it's too large to
            // fit inline, and we expect the original program to guarantee that
            // string lengths are accurate.
            unsafe { std::slice::from_raw_parts(self.text.pointer, self.length) }
        };
        T::decode(characters).map(|cow| cow.into_owned())
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
