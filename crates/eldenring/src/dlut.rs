use std::{
    hint::assert_unchecked,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    slice,
};

use bitfield::bitfield;
use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait DLReferenceCountObjectVmt {
    /// Ran when the ref count hits 0?
    fn clean_up(&self);

    fn destructor(&mut self);
}

/// Tracks the amount of references for the deriving class.
///
/// Source of name: RTTI
#[repr(C)]
pub struct DLReferenceCountObjectBase {
    pub vftable: VPtr<dyn DLReferenceCountObjectVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
}

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct PackedDate(u64);
    impl Debug;
    u16;
    pub year, set_year: 11, 0;
    pub millisecond, set_millisecond: 21, 12;
    u8;
    pub month, set_month: 25, 22;
    pub day_of_week, set_day_of_week: 28, 26;
    pub day, set_day: 33, 29;
    pub hours, set_hours: 38, 34;
    pub minutes, set_minutes: 44, 39;
    pub seconds, set_seconds: 50, 45;
    pub is_utc, set_is_utc: 51;
}

#[repr(C)]
/// Source of name: [dantelion2 leak].
///
/// [dantelion2 leak]: https://archive.org/details/dantelion2
pub struct DLDateTime {
    /// Uses FILETIME on windows
    /// (100-nanosecond intervals since January 1, 1601 UTC)
    pub time64: u64,
    /// Packed datetime value.
    pub date: PackedDate,
}

impl DLDateTime {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        is_utc: bool,
    ) -> Self {
        let mut date = PackedDate::default();
        date.set_year(year);
        date.set_month(month);
        date.set_day(day);
        date.set_hours(hours);
        date.set_minutes(minutes);
        date.set_seconds(seconds);
        date.set_millisecond(milliseconds);
        date.set_is_utc(is_utc);

        let time64 =
            Self::calculate_time64(year, month, day, hours, minutes, seconds, milliseconds);

        Self { time64, date }
    }

    pub fn year(&self) -> u16 {
        self.date.year()
    }

    pub fn month(&self) -> u8 {
        self.date.month()
    }

    pub fn day(&self) -> u8 {
        self.date.day()
    }

    pub fn hours(&self) -> u8 {
        self.date.hours()
    }

    pub fn minutes(&self) -> u8 {
        self.date.minutes()
    }

    pub fn seconds(&self) -> u8 {
        self.date.seconds()
    }

    pub fn is_utc(&self) -> bool {
        self.date.is_utc()
    }

    const fn calculate_time64(
        year: u16,
        month: u8,
        day: u8,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
    ) -> u64 {
        const fn is_leap_year(year: u16) -> bool {
            (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
        }
        const fn days_since_1601(year: u16, month: u8, day: u8) -> i64 {
            const DAYS_BEFORE_MONTH: [i64; 13] =
                [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
            let mut days = (year as i64 - 1601) * 365;
            days +=
                (year as i64 - 1601) / 4 - (year as i64 - 1601) / 100 + (year as i64 - 1601) / 400;
            days += DAYS_BEFORE_MONTH[month as usize];
            days += day as i64 - 1;
            if is_leap_year(year) && month > 2 {
                days += 1;
            }
            days
        }

        // Convert to FILETIME format (100-nanosecond intervals since January 1, 1601)
        const INTERVALS_PER_SECOND: u64 = 10_000_000;
        const INTERVALS_PER_MILLISECOND: u64 = 10_000;

        let days_since_1601 = days_since_1601(year, month, day);
        let total_seconds = (days_since_1601 as u64 * 86400)
            + (hours as u64 * 3600)
            + (minutes as u64 * 60)
            + (seconds as u64);

        total_seconds * INTERVALS_PER_SECOND + (milliseconds as u64 * INTERVALS_PER_MILLISECOND)
    }
}

#[repr(C)]
// A container with a fixed number of elements stored inline without an additional heap allocation
pub struct DLFixedVector<T, const C: usize> {
    elements: [MaybeUninit<T>; C],
    unk1: usize,
    checked_len: usize,
}

impl<T, const C: usize> Default for DLFixedVector<T, C> {
    fn default() -> Self {
        Self {
            elements: [const { MaybeUninit::uninit() }; C],
            unk1: 0,
            checked_len: 0,
        }
    }
}

impl<T, const C: usize> DLFixedVector<T, C> {
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    pub const fn capacity(&self) -> usize {
        C
    }

    pub fn as_slice(&self) -> &'_ [T] {
        unsafe {
            // Safety: enforced by `push()` and `truncate()`
            assert_unchecked(self.checked_len <= self.capacity());

            // Safety: elements up to `self.checked_len` are initialized
            slice::from_raw_parts(self.elements[0].as_ptr(), self.checked_len)
        }
    }

    pub fn as_mut_slice(&mut self) -> &'_ mut [T] {
        unsafe {
            // Safety: enforced by `push()` and `truncate()`
            assert_unchecked(self.checked_len <= self.capacity());

            // Safety: elements up to `self.checked_len` are initialized
            slice::from_raw_parts_mut(self.elements[0].as_mut_ptr(), self.checked_len)
        }
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    // Appends an element if there is sufficient spare capacity, otherwise an error is returned
    // with the element.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        let prev_len = self.len();
        if prev_len + 1 > self.capacity() {
            return Err(value);
        }

        self.elements[prev_len] = MaybeUninit::new(value);
        self.checked_len = prev_len + 1;
        Ok(())
    }

    // Truncates the vector to the given length, dropping elements that should no longer be
    // initialized.
    pub fn truncate(&mut self, new_len: usize) {
        let prev_len = self.len();
        if new_len < prev_len {
            for i in new_len..prev_len {
                // Safety: elements up to `self.checked_len` are initialized
                unsafe { self.elements[i].assume_init_drop() };
            }
            self.checked_len = new_len;
        }
    }
}

impl<T: Clone, const C: usize> DLFixedVector<T, C> {
    // Grows or shrinks the vector to the given length, initializing new elements with `value`,
    // or return an error with the value if there is insufficient capacity.
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), T> {
        if new_len > self.capacity() {
            return Err(value);
        }

        if new_len < self.len() {
            self.truncate(new_len);
        } else {
            for i in self.len()..new_len {
                self.elements[i] = MaybeUninit::new(value.clone());
            }
            self.checked_len = new_len;
        }

        Ok(())
    }
}

impl<T, const C: usize> Index<usize> for DLFixedVector<T, C> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.as_slice().index(index)
    }
}

impl<T, const C: usize> IndexMut<usize> for DLFixedVector<T, C> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.as_mut_slice().index_mut(index)
    }
}

impl<T, const C: usize> Drop for DLFixedVector<T, C> {
    fn drop(&mut self) {
        self.truncate(0);
    }
}
