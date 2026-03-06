use crate::AllocatorExt;
use crate::allocator::Allocator;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

/// MSVC's `std::basic_string<C, char_traits<C>, A>` on x64.
///
/// # Small String Optimization (SSO)
///
/// The inline buffer is always 16 bytes. Capacity in code units:
///```text
/// ┌──────────────┬───────┬─────────┬───────────────────┐
/// │    Alias     │   C   │ SSO_CAP │ Max inline length │
/// ├──────────────┼───────┼─────────┼───────────────────┤
/// │ NarrowString │  u8   │   16    │   15 code units   │
/// │ U8String     │  u8   │   16    │   15 code units   │
/// │ WideString   │  u16  │   8     │   7  code units   │
/// │ U16String    │  u16  │   8     │   7  code units   │
/// │ U32String    │  u32  │   4     │   3  code units   │
/// └──────────────┴───────┴─────────┴───────────────────┘
///```
/// When `capacity < SSO_CAP` the data lives inline. Otherwise, `buffer`
/// holds a heap pointer
#[repr(C)]
pub struct BasicString<C, A>
where
    C: CodeUnit,
    A: Allocator,
{
    #[cfg(not(feature = "msvc2012"))]
    allocator: A,
    buffer: StringBuffer<C>,
    size: usize,
    capacity: usize,
    #[cfg(feature = "msvc2012")]
    allocator: A,
}

pub trait CodeUnit: Copy + Default + 'static {
    const SSO_CAP: usize;
    type InlineBuffer: AsRef<[Self]> + AsMut<[Self]> + Default;
}

impl CodeUnit for u8 {
    const SSO_CAP: usize = 16;
    type InlineBuffer = [Self; 16];
}

impl CodeUnit for u16 {
    const SSO_CAP: usize = 8;
    type InlineBuffer = [Self; 8];
}

impl CodeUnit for u32 {
    const SSO_CAP: usize = 4;
    type InlineBuffer = [Self; 4];
}

/// Small String Optimization union.
///
/// # Safety
///
/// [`sso`] is only valid when `capacity < SSO_CAP`
///
/// [`heap`] is only valid when `capacity >= SSO_CAP`
///
/// The active variant is determined solely by [`BasicString::capacity`], there is no
/// discriminant. Reading the wrong variant is immediate UB
#[repr(C)]
union StringBuffer<C: CodeUnit> {
    inline: ManuallyDrop<C::InlineBuffer>,
    pointer: NonNull<C>,
}

impl<C, A> BasicString<C, A>
where
    C: CodeUnit,
    A: Allocator,
{
    /// Creates an empty string backed by `allocator`.
    ///
    /// Equivalent to `std::basic_string<C>()` with a custom allocator.
    /// Starts in SSO mode with `capacity = SSO_CAP - 1`
    pub fn new_in(allocator: A) -> Self {
        Self {
            // SAFETY: A zero-initialized SSO buffer is valid, it represents
            // an empty string. `capacity = SSO_CAP - 1` keeps us in SSO mode
            buffer: StringBuffer {
                inline: ManuallyDrop::new(C::InlineBuffer::default()),
            },
            size: 0,
            capacity: C::SSO_CAP - 1,
            allocator,
        }
    }

    /// Creates a string from a slice of code units, allocating if the
    /// slice does not fit in the SSO buffer.
    ///
    /// Equivalent to `std::basic_string<C>(ptr, len, allocator)`
    pub fn from_units_in(chars: impl AsRef<[C]>, mut allocator: A) -> Self {
        let chars = chars.as_ref();
        let len = chars.len();

        if len < C::SSO_CAP {
            let mut sso = C::InlineBuffer::default();
            sso.as_mut()[..len].copy_from_slice(chars);
            Self {
                buffer: StringBuffer {
                    inline: ManuallyDrop::new(sso),
                },
                size: len,
                capacity: C::SSO_CAP - 1,
                allocator,
            }
        } else {
            let ptr = unsafe { allocator.allocate_n::<C>(len + 1).cast::<C>() };
            unsafe {
                std::ptr::copy_nonoverlapping(chars.as_ptr(), ptr.as_ptr(), len);
                ptr.as_ptr().add(len).write(C::default());
            }
            Self {
                buffer: StringBuffer { pointer: ptr },
                size: len,
                capacity: len,
                allocator,
            }
        }
    }

    pub fn assign(&mut self, units: impl AsRef<[C]>) {
        self.clear();
        self.push_slice(units);
    }

    /// Returns the number of code units (not bytes, not Unicode scalar values)
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the allocated capacity in code units, excluding the NUL terminator
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a pointer to the first code unit, equivalent to `c_str()`.
    ///
    /// # Safety
    ///
    /// The pointer is valid for `self.len()` initialized code units and is
    /// followed by a NUL terminator. It must not outlive `self`
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const C {
        // SAFETY: The SSO/heap variant is selected by the capacity invariant,
        // which is maintained by all constructors and mutations
        if self.is_sso() {
            unsafe { (*std::ptr::addr_of!(self.buffer.inline)).as_ref().as_ptr() }
        } else {
            unsafe { self.buffer.pointer.as_ptr() }
        }
    }

    /// Returns a mutable raw pointer to the first code unit.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut C {
        if self.is_sso() {
            unsafe {
                (*std::ptr::addr_of_mut!(self.buffer.inline))
                    .as_mut()
                    .as_mut_ptr()
            }
        } else {
            unsafe { self.buffer.pointer.as_ptr() }
        }
    }

    /// Returns the string data as a slice of code units
    #[inline]
    pub fn as_code_units(&self) -> &[C] {
        // SAFETY: `as_ptr` returns a pointer to `self.size` initialized code
        // units valid for the lifetime of `self`
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.size) }
    }

    /// Returns the string data as a mutable slice of code units
    #[inline]
    pub fn as_mut_code_units(&mut self) -> &mut [C] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.size) }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as _, self.size * size_of::<C>()) }
    }
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_ptr() as _, self.size * size_of::<C>())
        }
    }

    /// Ensures capacity for at least `additional` more code units without
    /// reallocation. Does not count the NUL terminator.
    ///
    /// Same as [`String::reserve`]
    pub fn reserve(&mut self, additional: usize) {
        let needed = self.size + additional;
        if needed <= self.capacity {
            return;
        }
        // Growth factor: at least 1.5x current capacity, minimum to satisfy request
        let new_cap = needed.max(self.capacity + self.capacity / 2 + 1);
        self.reallocate(new_cap);
    }

    fn reallocate(&mut self, new_cap: usize) {
        let new_ptr = unsafe { self.allocator.allocate_n::<C>(new_cap + 1).cast::<C>() };
        // Copy existing data + NUL terminator in one shot
        unsafe {
            std::ptr::copy_nonoverlapping(self.as_ptr(), new_ptr.as_ptr(), self.size + 1);
        }
        if !self.is_sso() {
            unsafe {
                self.allocator
                    .deallocate_raw(self.buffer.pointer.as_ptr() as _)
            };
        }
        self.buffer.pointer = new_ptr;
        self.capacity = new_cap;
    }

    /// Appends a single code unit.
    ///
    /// Same as [`String::push`]
    pub fn push(&mut self, c: C) {
        self.reserve(1);
        unsafe {
            let ptr = self.as_mut_ptr();
            ptr.add(self.size).write(c);
            ptr.add(self.size + 1).write(C::default()); // NUL
        }
        self.size += 1;
    }

    /// Appends a slice of code units.
    ///
    /// Same as [`String::push_str`]
    pub fn push_slice(&mut self, other: impl AsRef<[C]>) {
        let other = other.as_ref();
        if other.is_empty() {
            return;
        }
        self.reserve(other.len());
        unsafe {
            let dst = self.as_mut_ptr().add(self.size);
            std::ptr::copy_nonoverlapping(other.as_ptr(), dst, other.len());
            dst.add(other.len()).write(C::default()); // NUL
        }
        self.size += other.len();
    }

    /// Shortens the string to `new_len` code units, dropping the rest.
    ///
    /// Panics if `new_len > len()`. Does not reallocate.
    ///
    /// Same as [`String::truncate`]
    pub fn truncate(&mut self, new_len: usize) {
        assert!(
            new_len <= self.size,
            "new_len ({new_len}) > len ({})",
            self.size
        );
        self.size = new_len;
        unsafe { self.as_mut_ptr().add(new_len).write(C::default()) };
    }

    /// Sets length to 0 without freeing the buffer.
    ///
    /// Same as [`String::clear`]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Returns `true` if the string data lives in the inline SSO buffer
    #[inline]
    fn is_sso(&self) -> bool {
        self.capacity < C::SSO_CAP
    }
}

impl<C, A> BasicString<C, A>
where
    C: CodeUnit + PartialEq,
    A: Allocator,
{
    /// Returns the index of the first occurrence of `needle`, or `None`.
    ///
    /// Same as [`str::find`]
    pub fn find(&self, needle: impl AsRef<[C]>) -> Option<usize> {
        let src = self.as_code_units();
        let needle = needle.as_ref();
        if needle.is_empty() {
            return Some(0);
        }
        src.windows(needle.len()).position(|w| w == needle)
    }

    /// Returns the index of the last occurrence of `needle`, or `None`.
    ///
    /// Same as [`str::rfind`]
    pub fn rfind(&self, needle: impl AsRef<[C]>) -> Option<usize> {
        let src = self.as_code_units();
        let needle = needle.as_ref();
        if needle.is_empty() {
            return Some(src.len());
        }
        src.windows(needle.len()).rposition(|w| w == needle)
    }

    /// Returns `true` if `needle` appears anywhere in the string.
    ///
    /// Same as [`str::contains`]
    #[inline]
    pub fn contains(&self, needle: impl AsRef<[C]>) -> bool {
        let needle = needle.as_ref();
        self.find(needle).is_some()
    }

    /// Returns `true` if the string starts with `prefix`.
    ///
    /// Same as [`str::starts_with`]
    #[inline]
    pub fn starts_with(&self, prefix: impl AsRef<[C]>) -> bool {
        let prefix = prefix.as_ref();
        let src = self.as_code_units();
        src.len() >= prefix.len() && &src[..prefix.len()] == prefix
    }

    /// Returns `true` if the string ends with `suffix`.
    ///
    /// Same as [`str::ends_with`]
    #[inline]
    pub fn ends_with(&self, suffix: impl AsRef<[C]>) -> bool {
        let suffix = suffix.as_ref();
        let src = self.as_code_units();
        let slen = suffix.len();
        src.len() >= slen && &src[src.len() - slen..] == suffix
    }

    /// Returns a new string with all occurrences of `from` replaced by `to`.
    ///
    /// Same as [`str::replace`]
    pub fn replace<T: AsRef<[C]>>(&self, from: T, to: T) -> Self {
        self.replacen(from, to, usize::MAX)
    }

    /// Returns a new string with the first `n` occurrences of `from` replaced by `to`.
    ///
    /// Same as [`str::replacen`]
    pub fn replacen<T: AsRef<[C]>>(&self, from: T, to: T, n: usize) -> Self {
        let from = from.as_ref();
        let to = to.as_ref();
        let mut out = Self::new_in(self.allocator.clone());
        let src = self.as_code_units();
        let flen = from.len();
        if flen == 0 || flen > src.len() || n == 0 {
            out.push_slice(src);
            return out;
        }
        let mut i = 0;
        let mut replaced = 0;
        while i + flen <= src.len() {
            if replaced < n && &src[i..i + flen] == from {
                out.push_slice(to);
                i += flen;
                replaced += 1;
            } else {
                out.push(src[i]);
                i += 1;
            }
        }
        if i < src.len() {
            out.push_slice(&src[i..]);
        }
        out
    }

    /// Splits at `index`, returning `(self[..index], self[index..])`.
    ///
    /// Panics if `index > len()`.
    ///
    /// Same as [`str::split_at`]
    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let src = self.as_code_units();
        assert!(index <= src.len(), "split index out of bounds");
        (
            Self::from_units_in(&src[..index], self.allocator.clone()),
            Self::from_units_in(&src[index..], self.allocator.clone()),
        )
    }

    /// Returns substrings split by `delimiter`, collected eagerly.
    ///
    /// Same as [`str::split`]
    pub fn split(&self, delimiter: impl AsRef<[C]>) -> Vec<Self> {
        let src = self.as_code_units();
        let delimiter = delimiter.as_ref();
        let dlen = delimiter.len();
        let mut parts = Vec::new();
        if dlen == 0 {
            parts.push(Self::from_units_in(src, self.allocator.clone()));
            return parts;
        }
        let mut start = 0;
        let mut i = 0;
        while i + dlen <= src.len() {
            if &src[i..i + dlen] == delimiter {
                parts.push(Self::from_units_in(&src[start..i], self.allocator.clone()));
                i += dlen;
                start = i;
            } else {
                i += 1;
            }
        }
        parts.push(Self::from_units_in(&src[start..], self.allocator.clone()));
        parts
    }

    /// Returns a new string with this string's content repeated `n` times.
    ///
    /// Same as [`str::repeat`]
    pub fn repeat(&self, n: usize) -> Self {
        let src = self.as_code_units();
        let mut out = Self::new_in(self.allocator.clone());
        out.reserve(src.len() * n);
        for _ in 0..n {
            out.push_slice(src);
        }
        out
    }
}

impl<A: Allocator> BasicString<u8, A> {
    /// Returns a new string with leading and trailing ASCII whitespace removed.
    ///
    /// Same as [`str::trim`]
    pub fn trim(&self) -> Self {
        let src = self.as_code_units();
        let start = src
            .iter()
            .position(|&c| !c.is_ascii_whitespace())
            .unwrap_or(src.len());
        let end = src
            .iter()
            .rposition(|&c| !c.is_ascii_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        Self::from_units_in(
            if start <= end { &src[start..end] } else { &[] },
            self.allocator.clone(),
        )
    }

    /// Returns a new string with leading ASCII whitespace removed.
    ///
    /// Same as [`str::trim_start`]
    pub fn trim_start(&self) -> Self {
        let src = self.as_code_units();
        let start = src
            .iter()
            .position(|&c| !c.is_ascii_whitespace())
            .unwrap_or(src.len());
        Self::from_units_in(&src[start..], self.allocator.clone())
    }

    /// Returns a new string with trailing ASCII whitespace removed.
    ///
    /// Same as [`str::trim_end`]
    pub fn trim_end(&self) -> Self {
        let src = self.as_code_units();
        let end = src
            .iter()
            .rposition(|&c| !c.is_ascii_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        Self::from_units_in(&src[..end], self.allocator.clone())
    }

    /// Converts all ASCII letters to uppercase in place.
    ///
    /// Same as [`str::make_ascii_uppercase`]
    pub fn make_ascii_uppercase(&mut self) {
        self.as_mut_code_units().make_ascii_uppercase();
    }

    /// Converts all ASCII letters to lowercase in place.
    ///
    /// Same as [`str::make_ascii_lowercase`]
    pub fn make_ascii_lowercase(&mut self) {
        self.as_mut_code_units().make_ascii_lowercase();
    }

    /// Returns a new string with all ASCII letters uppercased.
    ///
    /// Same as [`str::to_ascii_uppercase`]
    pub fn to_ascii_uppercase(&self) -> Self {
        let mut s = Self::from_units_in(self.as_code_units(), self.allocator.clone());
        s.make_ascii_uppercase();
        s
    }

    /// Returns a new string with all ASCII letters lowercased.
    ///
    /// Same as [`str::to_ascii_lowercase`]
    pub fn to_ascii_lowercase(&self) -> Self {
        let mut s = Self::from_units_in(self.as_code_units(), self.allocator.clone());
        s.make_ascii_lowercase();
        s
    }

    /// Interprets the contents as UTF-8 and returns a `&str`, or `None` if invalid
    #[inline]
    pub fn to_str(&self) -> Option<&str> {
        std::str::from_utf8(self.as_code_units()).ok()
    }
}

impl<C, A> Drop for BasicString<C, A>
where
    C: CodeUnit,
    A: Allocator,
{
    fn drop(&mut self) {
        if !self.is_sso() {
            unsafe {
                self.allocator
                    .deallocate_raw(self.buffer.pointer.as_ptr() as _)
            };
        }
    }
}

/// Single generic impl covers: `&[C]`, `[C; N]`, `&[C; N]`, `Vec<C>`,
/// other `BasicString` instances (cross-allocator), and for `C = u8`:
/// `str`, `&str`, `String` all via their `AsRef<[u8]>` impls
impl<C, A, S> PartialEq<S> for BasicString<C, A>
where
    C: CodeUnit + PartialEq,
    A: Allocator,
    S: AsRef<[C]>,
{
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.as_code_units() == other.as_ref()
    }
}

impl<C, A> Eq for BasicString<C, A>
where
    C: CodeUnit + PartialEq,
    A: Allocator,
{
}

impl<C, A, S> PartialOrd<S> for BasicString<C, A>
where
    C: CodeUnit + PartialEq + Ord,
    A: Allocator,
    S: AsRef<[C]>,
{
    #[inline]
    fn partial_cmp(&self, other: &S) -> Option<std::cmp::Ordering> {
        Some(self.as_code_units().cmp(other.as_ref()))
    }
}

impl<C, A> Ord for BasicString<C, A>
where
    C: CodeUnit + PartialEq + Ord,
    A: Allocator,
{
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_code_units().cmp(other.as_code_units())
    }
}

/// Hashes only the code units, so strings with identical content hash equally
/// regardless of allocator, capacity, or SSO vs heap storage.
impl<C, A> Hash for BasicString<C, A>
where
    C: CodeUnit + Hash,
    A: Allocator,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_code_units().hash(state);
    }
}

impl<A: Allocator> fmt::Debug for BasicString<u8, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match std::str::from_utf8(self.as_code_units()) {
            Ok(s) => write!(f, "{s:?}"),
            Err(_) => write!(f, "<invalid utf-8: {:02x?}>", self.as_code_units()),
        }
    }
}

impl<A: Allocator> fmt::Display for BasicString<u8, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match std::str::from_utf8(self.as_code_units()) {
            Ok(s) => f.write_str(s),
            Err(_) => write!(f, "<invalid utf-8>"),
        }
    }
}

impl<A: Allocator> fmt::Debug for BasicString<u16, A> {
    /// Displays printable characters as glyphs and non-printable ones as `\u{XXXX}`.
    /// Lone surrogates (invalid UTF-16) are shown as `\x{FFFD(XXXX)}`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WideString[\"")?;
        for r in char::decode_utf16(self.as_code_units().iter().cloned()) {
            match r {
                Ok(c) if !c.is_control() => write!(f, "{c}")?,
                Ok(c) => write!(f, "\\u{{{:04X}}}", c as u32)?,
                Err(s) => write!(f, "\\x{{FFFD({:04X})}}", s.unpaired_surrogate())?,
            }
        }
        write!(f, "\"]")
    }
}

impl<C, A> AsRef<[C]> for BasicString<C, A>
where
    C: CodeUnit,
    A: Allocator,
{
    #[inline]
    fn as_ref(&self) -> &[C] {
        self.as_code_units()
    }
}

impl<C, A> std::borrow::Borrow<[C]> for BasicString<C, A>
where
    C: CodeUnit,
    A: Allocator,
{
    #[inline]
    fn borrow(&self) -> &[C] {
        self.as_code_units()
    }
}
