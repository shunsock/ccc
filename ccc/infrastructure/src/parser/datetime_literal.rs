use domain::ast::Expression;
use domain::error::CccError;
use domain::time::{EpochSeconds, UtcOffset};

use super::pest_based_parser::Rule;

/// Build a datetime literal from `YYYY-MM-DDTHH:MM:SS` with optional timezone offset.
/// Field positions are fixed by the grammar, so slicing by index is safe.
pub(super) fn build_datetime_literal(
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let source = pair.as_str();

    // Parse date part: YYYY-MM-DD
    let year = source[0..4]
        .parse::<i64>()
        .map_err(|e| CccError::parse(format!("invalid datetime year: {e}")))?;
    let month = source[5..7]
        .parse::<u8>()
        .map_err(|e| CccError::parse(format!("invalid datetime month: {e}")))?;
    let day = source[8..10]
        .parse::<u8>()
        .map_err(|e| CccError::parse(format!("invalid datetime day: {e}")))?;

    // Parse time part: HH:MM:SS (after 'T')
    let hour = source[11..13]
        .parse::<u8>()
        .map_err(|e| CccError::parse(format!("invalid datetime hour: {e}")))?;
    let minute = source[14..16]
        .parse::<u8>()
        .map_err(|e| CccError::parse(format!("invalid datetime minute: {e}")))?;
    let second = source[17..19]
        .parse::<u8>()
        .map_err(|e| CccError::parse(format!("invalid datetime second: {e}")))?;

    // Parse optional timezone offset
    let offset = if source.len() > 19 {
        parse_timezone_offset(&source[19..])?
    } else {
        UtcOffset::UTC
    };

    // Validate via chrono: attempt to build an instant
    if EpochSeconds::from_calendar(year, month, day, hour, minute, second).is_none() {
        return Err(CccError::parse(format!(
            "invalid datetime: {year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
        )));
    }

    Ok(Expression::DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        offset,
    })
}

/// Parse a timezone suffix: `Z`, `+HH:MM`, `+HHMM`, or `+HH` (and `-` variants).
fn parse_timezone_offset(tz: &str) -> Result<UtcOffset, CccError> {
    if tz == "Z" {
        return Ok(UtcOffset::UTC);
    }
    let sign: i32 = if tz.starts_with('+') { 1 } else { -1 };
    let rest = &tz[1..];
    let hours = rest[0..2]
        .parse::<i32>()
        .map_err(|e| CccError::parse(format!("invalid timezone offset hours: {e}")))?;
    let minutes = if rest.len() > 2 {
        // Skip the colon if present
        let min_str = if rest.as_bytes()[2] == b':' {
            &rest[3..5]
        } else {
            &rest[2..4]
        };
        min_str
            .parse::<i32>()
            .map_err(|e| CccError::parse(format!("invalid timezone offset minutes: {e}")))?
    } else {
        0
    };
    let total_seconds = sign * (hours * 3600 + minutes * 60);
    UtcOffset::from_seconds(total_seconds).ok_or_else(|| {
        CccError::parse(format!(
            "timezone offset out of range: {tz} (must be within ±24:00)"
        ))
    })
}
