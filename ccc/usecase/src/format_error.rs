use domain::error::CccError;

/// Format an error with an optional caret indicator pointing to the error position.
///
/// When the error includes position info, produces output like:
/// ```text
///   2 + + 3
///       ^
///   error: expected number, function call, or '('
/// ```
pub fn format_error_with_caret(input: &str, error: &CccError) -> String {
    let position = match error {
        CccError::Parse {
            position: Some(pos),
            ..
        }
        | CccError::TypeCheck {
            position: Some(pos),
            ..
        } => Some(pos.column),
        _ => None,
    };

    let error_message = format!("error: {error}");

    match position {
        Some(column) => {
            // column is 1-based; caret offset is column - 1 spaces
            let caret_offset = column.saturating_sub(1);
            let caret_line = format!("{:>width$}^", "", width = caret_offset);
            format!("  {input}\n  {caret_line}\n  {error_message}")
        }
        None => format!("  {error_message}"),
    }
}
