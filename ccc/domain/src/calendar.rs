use chrono::{Datelike, NaiveDate, TimeZone, Timelike};

/// Returns the number of days in the given month (1-indexed) of the given year.
/// Returns `None` if the month is out of range or the year is out of chrono's range.
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    let year_i32 = i32::try_from(year).ok()?;
    if month == 12 {
        let start = NaiveDate::from_ymd_opt(year_i32, 12, 1)?;
        let end = NaiveDate::from_ymd_opt(year_i32 + 1, 1, 1)?;
        Some((end - start).num_days() as u8)
    } else {
        let start = NaiveDate::from_ymd_opt(year_i32, month as u32, 1)?;
        let end = NaiveDate::from_ymd_opt(year_i32, month as u32 + 1, 1)?;
        Some((end - start).num_days() as u8)
    }
}

/// Converts calendar components to epoch seconds (seconds since 1970-01-01T00:00:00 UTC).
/// Returns `None` if the date/time components are invalid.
pub fn calendar_to_epoch_seconds(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> Option<i64> {
    let year_i32 = i32::try_from(year).ok()?;
    let naive = NaiveDate::from_ymd_opt(year_i32, month as u32, day as u32)?.and_hms_opt(
        hour as u32,
        minute as u32,
        second as u32,
    )?;
    let utc = chrono::Utc.from_utc_datetime(&naive);
    Some(utc.timestamp())
}

/// Converts epoch seconds to calendar components (year, month, day, hour, minute, second).
pub fn epoch_seconds_to_calendar(epoch_seconds: i64) -> (i64, u8, u8, u8, u8, u8) {
    let dt = chrono::DateTime::from_timestamp(epoch_seconds, 0)
        .expect("epoch seconds out of chrono range");
    (
        dt.year() as i64,
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second() as u8,
    )
}
