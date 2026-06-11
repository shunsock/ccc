use chrono::{Datelike, NaiveDate, TimeZone, Timelike};

use super::duration_seconds::DurationSeconds;

/// A point in time as seconds since 1970-01-01T00:00:00Z.
///
/// Invariant: the value is representable by chrono, so calendar conversion
/// and display formatting never fail. Constructors return `None` instead of
/// letting an unrepresentable instant exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpochSeconds(i64);

impl EpochSeconds {
    /// Returns `None` if the instant is outside chrono's representable range.
    pub fn from_seconds(seconds: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp(seconds, 0).map(|_| Self(seconds))
    }

    /// Builds an instant from calendar components interpreted as UTC.
    /// Returns `None` for invalid dates (e.g. Feb 30) or out-of-range years.
    pub fn from_calendar(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Option<Self> {
        let year_i32 = i32::try_from(year).ok()?;
        let naive = NaiveDate::from_ymd_opt(year_i32, month as u32, day as u32)?.and_hms_opt(
            hour as u32,
            minute as u32,
            second as u32,
        )?;
        let utc = chrono::Utc.from_utc_datetime(&naive);
        Some(Self(utc.timestamp()))
    }

    /// Calendar components (year, month, day, hour, minute, second) in UTC.
    /// Infallible thanks to the construction invariant.
    pub fn to_calendar(self) -> (i64, u8, u8, u8, u8, u8) {
        let dt = chrono::DateTime::from_timestamp(self.0, 0)
            .expect("EpochSeconds invariant guarantees chrono range");
        (
            dt.year() as i64,
            dt.month() as u8,
            dt.day() as u8,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
        )
    }

    pub fn seconds(self) -> i64 {
        self.0
    }

    /// Moves forward by a span. `None` if the result leaves chrono's range.
    pub fn checked_add(self, delta: DurationSeconds) -> Option<Self> {
        self.0
            .checked_add(delta.seconds())
            .and_then(Self::from_seconds)
    }

    /// Moves backward by a span. `None` if the result leaves chrono's range.
    pub fn checked_sub(self, delta: DurationSeconds) -> Option<Self> {
        self.0
            .checked_sub(delta.seconds())
            .and_then(Self::from_seconds)
    }

    /// Signed span from `other` to `self`.
    pub fn duration_since(self, other: EpochSeconds) -> DurationSeconds {
        DurationSeconds::from_seconds(self.0 - other.0)
    }
}
