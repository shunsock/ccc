use domain::ast::Expression;
use domain::error::CccError;

use super::pest_based_parser::{Rule, build_expression};

/// Unwrap an atom (parenthesized expression, literal, or function call).
pub(super) fn build_atom(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| CccError::parse("empty atom".to_string()))?;
    build_expression(inner)
}

/// Build an integer or float literal. The presence of `.` decides the type.
pub(super) fn build_number(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
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

pub(super) fn build_function_call(
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let mut inner = pair.into_inner();
    let name = inner
        .next()
        .ok_or_else(|| CccError::parse("expected function name".to_string()))?
        .as_str()
        .to_string();

    let arguments_pair = inner
        .next()
        .ok_or_else(|| CccError::parse("expected arguments".to_string()))?;

    let arguments: Result<Vec<Expression>, CccError> =
        arguments_pair.into_inner().map(build_expression).collect();

    Ok(Expression::FunctionCall {
        name,
        arguments: arguments?,
    })
}

pub(super) fn build_list(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
    let elements: Result<Vec<Expression>, CccError> =
        pair.into_inner().map(build_expression).collect();
    Ok(Expression::List(elements?))
}
