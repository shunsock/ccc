use domain::ast::Expression;
use domain::error::CccError;

use super::pest_based_parser::Rule;

/// Build a duration literal from `HH:MM:SS` or `MM:SS` syntax.
pub(super) fn build_duration_literal(
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let source = pair.as_str();
    let parts: Vec<&str> = source.split(':').collect();

    let (hours, minutes, seconds) = if parts.len() == 2 {
        // MM:SS format
        let minutes = parts[0]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration minutes: {e}")))?;
        let seconds = parts[1]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration seconds: {e}")))?;
        (0i64, minutes, seconds)
    } else {
        // HH:MM:SS format
        let hours = parts[0]
            .parse::<i64>()
            .map_err(|e| CccError::parse(format!("invalid duration hours: {e}")))?;
        let minutes = parts[1]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration minutes: {e}")))?;
        let seconds = parts[2]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration seconds: {e}")))?;
        (hours, minutes, seconds)
    };

    if minutes >= 60 {
        return Err(CccError::parse(format!(
            "duration minutes out of range: {minutes} (must be 0-59)"
        )));
    }
    if seconds >= 60 {
        return Err(CccError::parse(format!(
            "duration seconds out of range: {seconds} (must be 0-59)"
        )));
    }
    Ok(Expression::DurationTime {
        hours,
        minutes,
        seconds,
    })
}
