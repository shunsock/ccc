use domain::error::CccError;

use super::pest_based_parser::Rule;

/// Convert a pest error into a CccError carrying a humanized message and column.
pub(super) fn to_parse_error(error: &pest::error::Error<Rule>) -> CccError {
    let column = match error.line_col {
        pest::error::LineColLocation::Pos((_, col)) => col,
        pest::error::LineColLocation::Span((_, col), _) => col,
    };
    CccError::parse_at(humanize_pest_error(&error.variant), column)
}

fn humanize_pest_error(variant: &pest::error::ErrorVariant<Rule>) -> String {
    match variant {
        pest::error::ErrorVariant::ParsingError {
            positives,
            negatives: _,
        } => {
            if positives.is_empty() {
                return "unexpected input".to_string();
            }
            let expected: Vec<&str> = positives.iter().map(humanize_rule).collect();
            format!("expected {}", expected.join(" or "))
        }
        pest::error::ErrorVariant::CustomError { message } => message.clone(),
    }
}

fn humanize_rule(rule: &Rule) -> &'static str {
    match rule {
        Rule::program => "expression",
        Rule::EOI => "end of input",
        Rule::expression => "expression",
        Rule::term => "term",
        Rule::power => "expression",
        Rule::unary => "number, function call, list, or '('",
        Rule::postfix => "number, function call, list, or '('",
        Rule::postfix_suffix => "'as' or '.method()'",
        Rule::method_call => ".method()",
        Rule::type_cast => "'as int' or 'as float'",
        Rule::cast_type => "'int' or 'float'",
        Rule::atom => "number, datetime, duration, function call, list, or '('",
        Rule::datetime_literal => "datetime (YYYY-MM-DDTHH:MM:SS)",
        Rule::timezone_offset => "timezone offset",
        Rule::duration_literal => "duration (HH:MM:SS)",
        Rule::list => "list",
        Rule::function_call => "function call",
        Rule::arguments => "arguments",
        Rule::number => "number",
        Rule::float => "number",
        Rule::integer => "number",
        Rule::identifier => "function name",
        Rule::additive_operator => "'+' or '-'",
        Rule::multiplicative_operator => "'*', '/', or '%'",
        Rule::power_operator => "'^'",
        Rule::unary_operator => "'+' or '-'",
        Rule::WHITESPACE => "whitespace",
    }
}
