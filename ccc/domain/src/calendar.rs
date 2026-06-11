use chrono::NaiveDate;

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
