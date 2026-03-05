use crate::AllocatorExt;
use crate::allocator::Allocator;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

/// MSVC's `std::basic_string<C, char_traits<C>, A>` on x64.
///
/// # Small String Optimisation (SSO)
///
/// The inline buffer is always 16 bytes. Capacity in code units:
///```text
/// ┌──────────────┬───────┬─────────┬───────────────────┐
/// │    Alias     │   C   │ SSO_CAP │ Max inline length │
/// ├──────────────┼───────┼─────────┼───────────────────┤
/// │ NarrowString │  u8   │   16    │   15 code units   │
/// │ Utf8String   │  u8   │   16    │   15 code units   │
/// │ WideString   │  u16  │   8     │   7  code units   │
/// │ Utf16String  │  u16  │   8     │   7  code units   │
/// │ Utf32String  │  u32  │   4     │   3  code units   │
/// └──────────────┴───────┴─────────┴───────────────────┘
///```
/// When `capacity < SSO_CAP` the data lives inline. Otherwise, `buffer`
/// holds a heap pointer
#[repr(C)]
pub struct BasicString<C, A, const SSO_CAP: usize>
where
    C: Copy + Default,
    A: Allocator + Clone,
{
    #[cfg(not(feature = "msvc2012"))]
    allocator: A,
    buffer: StringBuffer<C, SSO_CAP>,
    size: usize,
    capacity: usize,
    #[cfg(feature = "msvc2012")]
    allocator: A,
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
union StringBuffer<C, const N: usize> {
    sso: ManuallyDrop<[C; N]>,
    heap: NonNull<C>,
}

/// MSVC `std::string` byte string (`ANSI` / `Shift-JIS `/ `ISO-8859-1` etc)
pub type NarrowString<A> = BasicString<u8, A, 16>;

/// MSVC `std::u8string` (C++20), explicitly UTF-8
/// Identical layout to [`NarrowString`]
#[cfg(not(feature = "msvc2012"))]
pub type Utf8String<A> = BasicString<u8, A, 16>;

/// MSVC `std::wstring`, `wchar_t` (`ushort` on Windows) string
pub type WideString<A> = BasicString<u16, A, 8>;

/// MSVC `std::u16string` (C++17), explicitly UTF-16LE
/// Identical layout to [`WideString`]
#[cfg(not(feature = "msvc2012"))]
pub type Utf16String<A> = BasicString<u16, A, 8>;

/// MSVC `std::u32string`, UTF-32 string
pub type Utf32String<A> = BasicString<u32, A, 4>;

impl<C, A, const SSO_CAP: usize> BasicString<C, A, SSO_CAP>
where
    C: Copy + Default,
    A: Allocator + Clone,
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
                sso: ManuallyDrop::new([C::default(); SSO_CAP]),
            },
            size: 0,
            capacity: SSO_CAP - 1,
            allocator,
        }
    }

    /// Creates a string from a slice of code units, allocating if the
    /// slice does not fit in the SSO buffer.
    ///
    /// Equivalent to `std::basic_string<C>(ptr, len, allocator)`
    pub fn from_bytes_in(chars: &[C], mut allocator: A) -> Self {
        let len = chars.len();

        if len < SSO_CAP {
            // Fits inline, copy directly into the SSO buffer
            let mut sso = [C::default(); SSO_CAP];
            sso[..len].copy_from_slice(chars);
            Self {
                buffer: StringBuffer {
                    sso: ManuallyDrop::new(sso),
                },
                size: len,
                capacity: SSO_CAP - 1,
                allocator,
            }
        } else {
            // Allocate len + 1 code units to leave room for the NUL terminator
            //
            // SAFETY: `allocate_array` returns a valid, non null, properly
            // aligned pointer to `(len + 1) * size_of::<C>()` bytes. We
            // initialise all `len` code units immediately and write a NUL at
            // index `len` before any other access
            let ptr = allocator.allocate_n::<C>(len + 1).cast::<C>();
            unsafe {
                std::ptr::copy_nonoverlapping(chars.as_ptr(), ptr.as_ptr(), len);
                ptr.as_ptr().add(len).write(C::default()); // NUL terminator
            }

            Self {
                buffer: StringBuffer { heap: ptr },
                size: len,
                capacity: len,
                allocator,
            }
        }
    }

    /// Returns `true` if the string data lives in the inline SSO buffer
    #[inline]
    pub fn is_sso(&self) -> bool {
        self.capacity < SSO_CAP
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
            unsafe { self.buffer.sso }.as_ptr()
        } else {
            unsafe { self.buffer.heap.as_ptr() }
        }
    }

    /// Returns the string data as a slice of code units
    #[inline]
    pub fn as_code_units(&self) -> &[C] {
        // SAFETY: `as_ptr` returns a pointer to `self.size` initialized code
        // units valid for the lifetime of `self`
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.size) }
    }
}
