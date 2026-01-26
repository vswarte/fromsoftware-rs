use bitfield::bitfield;

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
