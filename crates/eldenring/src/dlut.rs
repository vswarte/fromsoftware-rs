use std::ops::{Deref, DerefMut};

use bitfield::bitfield;
use vtable_rs::VPtr;

use windows::Win32::Foundation::{FILETIME, SYSTEMTIME};
use windows::Win32::System::Time::FileTimeToSystemTime;

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
/// Source of name: dantelion2 leak
/// https://archive.org/details/dantelion2
pub struct DLDateTime {
    /// Set to FILETIME on creation.
    pub time64: FILETIME,
    /// Packed datetime value.
    pub date: PackedDate,
}

impl Deref for DLDateTime {
    type Target = PackedDate;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
}

impl DerefMut for DLDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.date
    }
}

impl DLDateTime {
    /// Creates a new `DLDateTime` from a `FILETIME`.
    pub fn new(time64: FILETIME, is_utc: bool) -> Self {
        Self::from_time64(time64, is_utc)
    }

    /// Creates a new `DLDateTime` from a `FILETIME`, converting the time
    /// components into a packed bitfield format.
    pub fn from_time64(time64: FILETIME, is_utc: bool) -> Self {
        let mut date = PackedDate::default();
        let mut system_time = SYSTEMTIME::default();

        if unsafe { FileTimeToSystemTime(&time64, &mut system_time) }.is_ok() {
            date.set_year(system_time.wYear);
            date.set_millisecond(system_time.wMilliseconds);
            date.set_month(system_time.wMonth as u8);
            date.set_day_of_week(system_time.wDayOfWeek as u8);
            date.set_day(system_time.wDay as u8);
            date.set_hours(system_time.wHour as u8);
            date.set_minutes(system_time.wMinute as u8);
            date.set_seconds(system_time.wSecond as u8);
            date.set_is_utc(is_utc);
        }

        Self { time64, date }
    }
}
