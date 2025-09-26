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

        let mut system_time = SYSTEMTIME {
            wYear: year,
            wMonth: month as u16,
            wDayOfWeek: 0,
            wDay: day as u16,
            wHour: hours as u16,
            wMinute: minutes as u16,
            wSecond: seconds as u16,
            wMilliseconds: milliseconds,
        };

        Self {
            time64: FILETIME::default(),
            date,
        }
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
}
