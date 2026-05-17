use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use encoding_rs::DecoderResult;
use fromsoftware_shared_stl::{BasicString, CodeUnit};
use thiserror::Error;

use crate::DLAllocatorForStl;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum DLCharacterSet {
    UTF8 = 0,
    #[default]
    UTF16 = 1,
    Iso8859_1 = 2,
    ShiftJis = 3,
    EucJp = 4,
    UTF32 = 5,
}

impl DLCharacterSet {
    /// Returns the [encoding_rs::Encoding] that corresponds to this character
    /// set, or None if no such encoding exists (which is only possible for
    /// [DLCharacterSet::UTF32]).
    pub fn encoding(&self) -> Option<&'static encoding_rs::Encoding> {
        Some(match self {
            DLCharacterSet::UTF8 => encoding_rs::UTF_8,
            #[cfg(target_endian = "little")]
            DLCharacterSet::UTF16 => encoding_rs::UTF_16LE,
            #[cfg(target_endian = "big")]
            DLCharacterSet::UTF16 => encoding_rs::UTF_16BE,
            DLCharacterSet::Iso8859_1 => encoding_rs::WINDOWS_1252,
            DLCharacterSet::ShiftJis => encoding_rs::SHIFT_JIS,
            DLCharacterSet::EucJp => encoding_rs::EUC_JP,
            DLCharacterSet::UTF32 => return None,
        })
    }
}

#[derive(Error, Debug)]
pub enum DLStringError {
    #[error("Failed to decode string")]
    DecodeError,
    #[error("Failed to encode string")]
    EncodeError,
}

mod seal {
    pub trait Sealed {}
}

pub trait DLStringKind: seal::Sealed {
    type Unit: CodeUnit + PartialEq + Hash + 'static;
    const ENCODING: DLCharacterSet;

    /// Encode a Rust `&str` into this kind's code units
    fn encode(s: &str) -> Result<Vec<Self::Unit>, DLStringError>;
    /// Decode a raw byte buffer (in this kind's encoding) to a UTF-8 `Cow<str>`
    fn decode(bytes: &[u8]) -> Result<Cow<'_, str>, DLStringError>;
}

macro_rules! def_kind {
    ($name:ident,  $unit:ty, $enc:expr) => {
        pub struct $name;
        impl seal::Sealed for $name {}
        impl DLStringKind for $name {
            type Unit = $unit;
            const ENCODING: DLCharacterSet = $enc;
            fn encode(s: &str) -> Result<Vec<$unit>, DLStringError> {
                encode_str(s, $enc)
            }
            fn decode(b: &[u8]) -> Result<Cow<'_, str>, DLStringError> {
                decode_bytes(b, $enc)
            }
        }
    };
}

def_kind!(DLUTF8StringKind, u8, DLCharacterSet::UTF8);
def_kind!(DLISO8859_1StringKind, u8, DLCharacterSet::Iso8859_1);
def_kind!(DLShiftJisStringKind, u8, DLCharacterSet::ShiftJis);
def_kind!(DLEucJpStringKind, u8, DLCharacterSet::EucJp);
def_kind!(DLUTF16StringKind, u16, DLCharacterSet::UTF16);
def_kind!(DLUTF32StringKind, u32, DLCharacterSet::UTF32);

fn encode_str<U: Copy>(s: &str, enc: DLCharacterSet) -> Result<Vec<U>, DLStringError> {
    // These transmutes are safe: each arm only executes when U is the
    // exact type the transmute targets (enforced by the def_kind! macro).
    match enc {
        DLCharacterSet::UTF16 => {
            let v: Vec<u16> = s.encode_utf16().collect();
            Ok(unsafe { std::mem::transmute::<Vec<u16>, Vec<U>>(v) })
        }
        DLCharacterSet::UTF32 => {
            let v: Vec<u32> = s.chars().map(|c| c as u32).collect();
            Ok(unsafe { std::mem::transmute::<Vec<u32>, Vec<U>>(v) })
        }
        DLCharacterSet::UTF8 => {
            let v = s.as_bytes().to_vec();
            Ok(unsafe { std::mem::transmute::<Vec<u8>, Vec<U>>(v) })
        }
        _ => {
            let (encoded, _, errors) = enc.encoding().unwrap().encode(s);
            if errors {
                return Err(DLStringError::EncodeError);
            }
            Ok(unsafe { std::mem::transmute::<Vec<u8>, Vec<U>>(encoded.into_owned()) })
        }
    }
}

fn decode_bytes(bytes: &[u8], enc: DLCharacterSet) -> Result<Cow<'_, str>, DLStringError> {
    match enc {
        DLCharacterSet::UTF16 => {
            if !bytes.len().is_multiple_of(2) {
                return Err(DLStringError::DecodeError);
            }
            let units = unsafe {
                std::slice::from_raw_parts(bytes.as_ptr() as *const u16, bytes.len() / 2)
            };
            char::decode_utf16(units.iter().cloned())
                .map(|r| r.map_err(|_| DLStringError::DecodeError))
                .collect::<Result<String, _>>()
                .map(Cow::Owned)
        }
        DLCharacterSet::UTF32 => {
            if !bytes.len().is_multiple_of(4) {
                return Err(DLStringError::DecodeError);
            }
            let units = unsafe {
                std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4)
            };
            units
                .iter()
                .map(|&c| char::from_u32(c).ok_or(DLStringError::DecodeError))
                .collect::<Result<String, _>>()
                .map(Cow::Owned)
        }
        DLCharacterSet::UTF8 => std::str::from_utf8(bytes)
            .map(Cow::Borrowed)
            .map_err(|_| DLStringError::DecodeError),
        _ => {
            let (cow, _, errors) = enc.encoding().unwrap().decode(bytes);
            if errors {
                Err(DLStringError::DecodeError)
            } else {
                Ok(cow)
            }
        }
    }
}

/// Compare a raw byte buffer (in `enc` encoding) against a UTF-8 string
/// without allocating
fn bytes_eq_str(bytes: &[u8], enc: DLCharacterSet, other: &str) -> bool {
    match enc {
        DLCharacterSet::UTF8 => bytes == other.as_bytes(),

        DLCharacterSet::UTF16 => {
            if !bytes.len().is_multiple_of(2) {
                return false;
            }
            let units = unsafe {
                std::slice::from_raw_parts(bytes.as_ptr() as *const u16, bytes.len() / 2)
            };
            let mut their = other.chars();
            for r in char::decode_utf16(units.iter().cloned()) {
                match (r, their.next()) {
                    (Ok(a), Some(b)) if a == b => {}
                    _ => return false,
                }
            }
            their.next().is_none()
        }

        DLCharacterSet::UTF32 => {
            if !bytes.len().is_multiple_of(4) {
                return false;
            }
            let units = unsafe {
                std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4)
            };
            let mut their = other.chars();
            for &cp in units {
                match (char::from_u32(cp), their.next()) {
                    (Some(a), Some(b)) if a == b => {}
                    _ => return false,
                }
            }
            their.next().is_none()
        }

        _ => {
            let mut decoder = enc.encoding().unwrap().new_decoder();
            match decoder.max_utf8_buffer_length_without_replacement(bytes.len()) {
                Some(m) if m >= other.len() => {}
                _ => return false,
            }
            let mut src = bytes;
            let mut their = other.as_bytes().iter();
            let mut buf = [0u8; 64];
            loop {
                let (result, read, written) =
                    decoder.decode_to_utf8_without_replacement(src, &mut buf, src.len() <= 64);
                if matches!(result, DecoderResult::Malformed(_, _)) {
                    return false;
                }
                for &b in &buf[..written] {
                    match their.next() {
                        Some(&t) if t == b => {}
                        _ => return false,
                    }
                }
                if matches!(result, DecoderResult::InputEmpty) {
                    return their.next().is_none();
                }
                src = &src[read..];
            }
        }
    }
}

#[repr(C)]
pub struct DLString<T: DLStringKind = DLUTF16StringKind> {
    base: BasicString<T::Unit, DLAllocatorForStl>,
    encoding: DLCharacterSet,
}

impl<T: DLStringKind> DLString<T> {
    fn decode_storage(&self) -> Result<Cow<'_, str>, DLStringError> {
        T::decode(self.as_bytes())
    }

    pub fn new(allocator: DLAllocatorForStl) -> Self {
        Self {
            base: BasicString::new_in(allocator),
            encoding: T::ENCODING,
        }
    }

    pub fn from_str(
        s: impl AsRef<str>,
        allocator: DLAllocatorForStl,
    ) -> Result<Self, DLStringError> {
        let units = T::encode(s.as_ref())?;
        Ok(Self {
            base: BasicString::from_units_in(&units, allocator),
            encoding: T::ENCODING,
        })
    }

    // Replaces the entire content by encoding a UTF-8 string.
    ///
    /// Accepts `&str`, `String`, `Cow<str>`, etc.
    /// Reuses the existing allocation if capacity is sufficient.
    pub fn assign_str(&mut self, s: impl AsRef<str>) -> Result<(), DLStringError> {
        let units = T::encode(s.as_ref())?;
        self.base.assign(&units);
        Ok(())
    }

    /// Decodes the stored bytes to an owned UTF-8 `String`
    pub fn to_string(&self) -> Result<String, DLStringError> {
        self.decode_storage().map(Cow::into_owned)
    }

    /// Transcodes from a `DLString` of a different kind.
    /// If the encodings match the bytes are copied directly without going
    /// through UTF-8
    pub fn transcode_from<U: DLStringKind>(
        other: &DLString<U>,
        allocator: DLAllocatorForStl,
    ) -> Result<Self, DLStringError> {
        if T::ENCODING == U::ENCODING {
            // Safety: T::Unit and U::Unit are guaranteed to be the same here
            let units: &[<T as DLStringKind>::Unit] =
                unsafe { std::mem::transmute(other.as_code_units()) };
            Ok(Self {
                base: BasicString::from_units_in(units, allocator),
                encoding: T::ENCODING,
            })
        } else {
            Self::from_str(other.decode_storage()?, allocator)
        }
    }

    #[inline]
    pub fn encoding(&self) -> DLCharacterSet {
        self.encoding
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    /// Returns `true` if the decoded content contains `needle`
    pub fn contains_str(&self, needle: impl AsRef<str>) -> bool {
        self.to_string().is_ok_and(|s| s.contains(needle.as_ref()))
    }

    /// Returns `true` if the decoded content starts with `prefix`
    pub fn starts_with_str(&self, prefix: impl AsRef<str>) -> bool {
        self.to_string()
            .is_ok_and(|s| s.starts_with(prefix.as_ref()))
    }

    /// Returns `true` if the decoded content ends with `suffix`
    pub fn ends_with_str(&self, suffix: impl AsRef<str>) -> bool {
        self.to_string().is_ok_and(|s| s.ends_with(suffix.as_ref()))
    }

    /// Returns the char index of the first occurrence of `needle`, or `None`
    pub fn find_str(&self, needle: impl AsRef<str>) -> Option<usize> {
        self.to_string().ok()?.find(needle.as_ref())
    }

    /// Returns a new `DLString` with all occurrences of `from` replaced by `to`
    pub fn replace_str(
        &self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        allocator: DLAllocatorForStl,
    ) -> Result<Self, DLStringError> {
        Self::from_str(
            self.to_string()?.replace(from.as_ref(), to.as_ref()),
            allocator,
        )
    }

    /// Returns a new `DLString` with the first occurrence of `from` replaced by `to`
    pub fn replace_first_str(
        &self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        allocator: DLAllocatorForStl,
    ) -> Result<Self, DLStringError> {
        Self::from_str(
            self.to_string()?.replacen(from.as_ref(), to.as_ref(), 1),
            allocator,
        )
    }

    /// Splits on `delimiter`, returning each part as a new `DLString`
    pub fn split_str(
        &self,
        delimiter: impl AsRef<str>,
        allocator: DLAllocatorForStl,
    ) -> Result<Vec<Self>, DLStringError> {
        self.to_string()?
            .split(delimiter.as_ref())
            .map(|part| Self::from_str(part, allocator.clone()))
            .collect()
    }

    /// Returns a new `DLString` with leading and trailing whitespace removed
    pub fn trim_str(&self, allocator: DLAllocatorForStl) -> Result<Self, DLStringError> {
        Self::from_str(self.to_string()?.trim(), allocator)
    }

    /// Returns a new `DLString` with the content uppercased (Unicode-aware)
    pub fn to_uppercase_str(&self, allocator: DLAllocatorForStl) -> Result<Self, DLStringError> {
        Self::from_str(self.to_string()?.to_uppercase(), allocator)
    }

    /// Returns a new `DLString` with the content lowercased (Unicode-aware)
    pub fn to_lowercase_str(&self, allocator: DLAllocatorForStl) -> Result<Self, DLStringError> {
        Self::from_str(self.to_string()?.to_lowercase(), allocator)
    }
}

impl<T: DLStringKind> TryFrom<(&str, DLAllocatorForStl)> for DLString<T> {
    type Error = DLStringError;
    fn try_from((s, alloc): (&str, DLAllocatorForStl)) -> Result<Self, Self::Error> {
        Self::from_str(s, alloc)
    }
}

impl<T: DLStringKind> Deref for DLString<T> {
    type Target = BasicString<T::Unit, DLAllocatorForStl>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: DLStringKind> DerefMut for DLString<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: DLStringKind> fmt::Display for DLString<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl<T: DLStringKind> fmt::Debug for DLString<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => write!(f, "DLString({:?}, {:?})", T::ENCODING, s),
            Err(_) => write!(f, "DLString({:?}, <decode error>)", T::ENCODING),
        }
    }
}

/// `DLString<T> == DLString<U>`: byte comparison when same encoding,
/// UTF-8 round-trip when different.
impl<T: DLStringKind, U: DLStringKind> PartialEq<DLString<U>> for DLString<T> {
    fn eq(&self, other: &DLString<U>) -> bool {
        if T::ENCODING == U::ENCODING {
            self.base.as_bytes() == other.base.as_bytes()
        } else {
            match (self.to_string(), other.to_string()) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            }
        }
    }
}

impl<T: DLStringKind> Eq for DLString<T> {}

/// `DLString == &str`, `DLString == String`, `DLString == Cow<str>`, etc.
///
/// For UTF-8/16/32 this is allocation-free.
/// For legacy encodings (Shift-JIS, EUC-JP, ISO-8859-1) it uses a
/// stack-allocated 64-byte decode buffer.
impl<T: DLStringKind, S: AsRef<str>> PartialEq<S> for DLString<T> {
    fn eq(&self, other: &S) -> bool {
        bytes_eq_str(self.base.as_bytes(), T::ENCODING, other.as_ref())
    }
}

impl<T: DLStringKind> Hash for DLString<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::ENCODING.hash(state);
        self.base.as_code_units().hash(state);
    }
}

#[repr(C)]
pub struct DLRawString<T: DLStringKind = DLUTF16StringKind> {
    vftable: usize,
    backing_string: Option<NonNull<T::Unit>>,
    pub length: usize,
    unk18: u32,
    pub char_size: u16,
    pub encoding: DLCharacterSet,
    pub flags: u8,
}

impl<T: DLStringKind> DLRawString<T> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn units(&self) -> &[T::Unit] {
        self.backing_string.map_or(&[], |ptr| unsafe {
            std::slice::from_raw_parts(ptr.as_ptr(), self.length)
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        let u = self.units();
        unsafe { std::slice::from_raw_parts(u.as_ptr() as *const u8, std::mem::size_of_val(u)) }
    }

    pub fn to_string(&self) -> Result<String, DLStringError> {
        T::decode(self.as_bytes()).map(Cow::into_owned)
    }

    pub fn contains_str(&self, needle: impl AsRef<str>) -> bool {
        self.to_string().is_ok_and(|s| s.contains(needle.as_ref()))
    }
    pub fn starts_with_str(&self, prefix: impl AsRef<str>) -> bool {
        self.to_string()
            .is_ok_and(|s| s.starts_with(prefix.as_ref()))
    }
    pub fn ends_with_str(&self, suffix: impl AsRef<str>) -> bool {
        self.to_string().is_ok_and(|s| s.ends_with(suffix.as_ref()))
    }
    pub fn find_str(&self, needle: impl AsRef<str>) -> Option<usize> {
        self.to_string().ok()?.find(needle.as_ref())
    }
}

impl<T: DLStringKind, S: AsRef<str>> PartialEq<S> for DLRawString<T> {
    fn eq(&self, other: &S) -> bool {
        bytes_eq_str(self.as_bytes(), T::ENCODING, other.as_ref())
    }
}

impl<T: DLStringKind, U: DLStringKind> PartialEq<DLRawString<U>> for DLRawString<T> {
    fn eq(&self, other: &DLRawString<U>) -> bool {
        if T::ENCODING == U::ENCODING {
            self.as_bytes() == other.as_bytes()
        } else {
            match (self.to_string(), other.to_string()) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            }
        }
    }
}

impl<T: DLStringKind> Eq for DLRawString<T> {}

impl<T: DLStringKind> fmt::Display for DLRawString<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl<T: DLStringKind> fmt::Debug for DLRawString<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => write!(f, "DLRawString({:?}, {:?})", T::ENCODING, s),
            Err(_) => write!(f, "DLRawString({:?}, <decode error>)", T::ENCODING),
        }
    }
}

pub type DLCodedString<T> = DLRawString<T>;

#[repr(C)]
pub struct DLInplaceStr<T: DLStringKind, const N: usize> {
    pub base: DLCodedString<T>,
    pub bytes: [T::Unit; N],
    unk: usize,
}

impl<T: DLStringKind, const N: usize> DLInplaceStr<T, N> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }
    pub fn to_string(&self) -> Result<String, DLStringError> {
        self.base.to_string()
    }
    pub fn contains_str(&self, n: impl AsRef<str>) -> bool {
        self.base.contains_str(n)
    }
    pub fn starts_with_str(&self, p: impl AsRef<str>) -> bool {
        self.base.starts_with_str(p)
    }
    pub fn ends_with_str(&self, s: impl AsRef<str>) -> bool {
        self.base.ends_with_str(s)
    }
    pub fn find_str(&self, n: impl AsRef<str>) -> Option<usize> {
        self.base.find_str(n)
    }
}

impl<T: DLStringKind, const N: usize, S: AsRef<str>> PartialEq<S> for DLInplaceStr<T, N> {
    fn eq(&self, other: &S) -> bool {
        self.base == other.as_ref()
    }
}

impl<T: DLStringKind, U: DLStringKind, const N: usize, const M: usize> PartialEq<DLInplaceStr<U, M>>
    for DLInplaceStr<T, N>
{
    fn eq(&self, other: &DLInplaceStr<U, M>) -> bool {
        self.base == other.base
    }
}

impl<T: DLStringKind, const N: usize> Eq for DLInplaceStr<T, N> {}

impl<T: DLStringKind, const N: usize> fmt::Display for DLInplaceStr<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.base.fmt(f)
    }
}

impl<T: DLStringKind, const N: usize> fmt::Debug for DLInplaceStr<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.base.fmt(f)
    }
}
