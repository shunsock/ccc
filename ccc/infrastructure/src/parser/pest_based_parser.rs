use pest::Parser as PestParserTrait;
use pest_derive::Parser as PestDerive;

use domain::ast::{AbstractSyntaxTree, BinaryOperation, Expression, UnaryOperation};
use domain::error::CccError;
use domain::interface::parser::CccParser;

#[derive(PestDerive)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

/// CccParser implementation backed by pest PEG parser.
pub struct PestBasedParser;

impl CccParser for PestBasedParser {
    fn parse(&self, input: &str) -> Result<AbstractSyntaxTree, CccError> {
        let pairs = ExpressionParser::parse(Rule::expression, input).map_err(|e| {
            let column = match e.line_col {
                pest::error::LineColLocation::Pos((_, col)) => Some(col),
                pest::error::LineColLocation::Span((_, col), _) => Some(col),
            };
            let message = Self::humanize_pest_error(&e.variant);
            match column {
                Some(col) => CccError::parse_at(message, col),
                None => CccError::parse(message),
            }
        })?;

        let pair = pairs
            .into_iter()
            .next()
            .ok_or_else(|| CccError::parse("empty input".to_string()))?;

        let expression = Self::build_expression(pair)?;
        Ok(AbstractSyntaxTree { expression })
    }
}

impl PestBasedParser {
    fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        match pair.as_rule() {
            Rule::expression => Self::build_binary_expression(pair, Self::to_additive_operation),
            Rule::term => Self::build_binary_expression(pair, Self::to_multiplicative_operation),
            Rule::power => Self::build_right_associative_expression(pair),
            Rule::unary => Self::build_unary(pair),
            Rule::atom => Self::build_atom(pair),
            Rule::number => Self::build_number(pair),
            Rule::function_call => Self::build_function_call(pair),
            Rule::datetime_literal => Self::build_datetime_literal(pair),
            Rule::duration_literal => Self::build_duration_literal(pair),
            Rule::list => Self::build_list(pair),
            _ => Err(CccError::parse(format!(
                "unexpected rule: {:?}",
                pair.as_rule()
            ))),
        }
    }

    fn build_binary_expression(
        pair: pest::iterators::Pair<Rule>,
        operation_mapper: fn(&pest::iterators::Pair<Rule>) -> BinaryOperation,
    ) -> Result<Expression, CccError> {
        let mut inner = pair.into_inner();
        let first = inner
            .next()
            .ok_or_else(|| CccError::parse("expected operand".to_string()))?;
        let mut left = Self::build_expression(first)?;

        while let Some(operator_pair) = inner.next() {
            let right_pair = inner
                .next()
                .ok_or_else(|| CccError::parse("expected right operand".to_string()))?;
            let operator = operation_mapper(&operator_pair);
            let right = Self::build_expression(right_pair)?;
            left = Expression::BinaryOperation {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Build right-associative binary expression for power operator.
    /// `2^3^2` becomes `2^(3^2)` instead of `(2^3)^2`.
    fn build_right_associative_expression(
        pair: pest::iterators::Pair<Rule>,
    ) -> Result<Expression, CccError> {
        let mut operands = Vec::new();
        let mut inner = pair.into_inner();

        let first = inner
            .next()
            .ok_or_else(|| CccError::parse("expected operand".to_string()))?;
        operands.push(Self::build_expression(first)?);

        while let Some(_operator_pair) = inner.next() {
            let right_pair = inner
                .next()
                .ok_or_else(|| CccError::parse("expected right operand".to_string()))?;
            operands.push(Self::build_expression(right_pair)?);
        }

        // Fold from right: [a, b, c] -> a^(b^c)
        let mut result = operands.pop().expect("at least one operand");
        while let Some(left) = operands.pop() {
            result = Expression::BinaryOperation {
                operator: BinaryOperation::Power,
                left: Box::new(left),
                right: Box::new(result),
            };
        }

        Ok(result)
    }

    fn build_unary(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let mut inner = pair.into_inner();
        let first = inner
            .next()
            .ok_or_else(|| CccError::parse("expected expression".to_string()))?;

        if first.as_rule() == Rule::unary_operator {
            let operand_pair = inner.next().ok_or_else(|| {
                CccError::parse("expected operand after unary operator".to_string())
            })?;
            let operator = match first.as_str() {
                "-" => UnaryOperation::Negate,
                _ => UnaryOperation::Positive,
            };
            let operand = Self::build_expression(operand_pair)?;
            Ok(Expression::UnaryOperation {
                operator,
                operand: Box::new(operand),
            })
        } else {
            Self::build_expression(first)
        }
    }

    fn build_atom(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let inner = pair
            .into_inner()
            .next()
            .ok_or_else(|| CccError::parse("empty atom".to_string()))?;
        Self::build_expression(inner)
    }

    fn build_number(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let source = pair.as_str();
        if source.contains('.') {
            let n = source
                .parse::<f64>()
                .map_err(|e| CccError::parse(e.to_string()))?;
            Ok(Expression::Float(n))
        } else {
            let n = source
                .parse::<i64>()
                .map_err(|e| CccError::parse(e.to_string()))?;
            Ok(Expression::Integer(n))
        }
    }

    fn build_function_call(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or_else(|| CccError::parse("expected function name".to_string()))?
            .as_str()
            .to_string();

        let arguments_pair = inner
            .next()
            .ok_or_else(|| CccError::parse("expected arguments".to_string()))?;

        let arguments: Result<Vec<Expression>, CccError> = arguments_pair
            .into_inner()
            .map(Self::build_expression)
            .collect();

        Ok(Expression::FunctionCall {
            name,
            arguments: arguments?,
        })
    }

    fn build_datetime_literal(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
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
        let offset_seconds = if source.len() > 19 {
            let tz_part = &source[19..];
            Self::parse_timezone_offset(tz_part)?
        } else {
            0 // default to UTC
        };

        // Validate via chrono: attempt to build a NaiveDateTime
        if domain::value::calendar_to_epoch_seconds(year, month, day, hour, minute, second)
            .is_none()
        {
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
            offset_seconds,
        })
    }

    fn parse_timezone_offset(tz: &str) -> Result<i32, CccError> {
        if tz == "Z" {
            return Ok(0);
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
        Ok(sign * (hours * 3600 + minutes * 60))
    }

    fn build_duration_literal(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let source = pair.as_str();
        let parts: Vec<&str> = source.split(':').collect();
        let hours = parts[0]
            .parse::<i64>()
            .map_err(|e| CccError::parse(format!("invalid duration hours: {e}")))?;
        let minutes = parts[1]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration minutes: {e}")))?;
        let seconds = parts[2]
            .parse::<u8>()
            .map_err(|e| CccError::parse(format!("invalid duration seconds: {e}")))?;
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

    fn build_list(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
        let elements: Result<Vec<Expression>, CccError> =
            pair.into_inner().map(Self::build_expression).collect();
        Ok(Expression::List(elements?))
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
                let expected: Vec<&str> = positives.iter().map(Self::humanize_rule).collect();
                format!("expected {}", expected.join(" or "))
            }
            pest::error::ErrorVariant::CustomError { message } => message.clone(),
        }
    }

    fn humanize_rule(rule: &Rule) -> &'static str {
        match rule {
            Rule::expression => "expression",
            Rule::term => "term",
            Rule::power => "expression",
            Rule::unary => "number, function call, list, or '('",
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

    fn to_additive_operation(pair: &pest::iterators::Pair<Rule>) -> BinaryOperation {
        match pair.as_str() {
            "+" => BinaryOperation::Add,
            "-" => BinaryOperation::Subtract,
            _ => BinaryOperation::Add,
        }
    }

    fn to_multiplicative_operation(pair: &pest::iterators::Pair<Rule>) -> BinaryOperation {
        match pair.as_str() {
            "*" => BinaryOperation::Multiply,
            "/" => BinaryOperation::Divide,
            "%" => BinaryOperation::Modulo,
            _ => BinaryOperation::Multiply,
        }
    }
}
