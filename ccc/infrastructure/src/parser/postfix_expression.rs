use domain::ast::{CastTargetType, Expression};
use domain::error::CccError;

use super::pest_based_parser::{Rule, build_expression};

/// Build a postfix chain: method calls (`.method()`) and type casts (`as type`).
pub(super) fn build_postfix(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
    let mut inner = pair.into_inner();
    let first = inner
        .next()
        .ok_or_else(|| CccError::parse("expected expression".to_string()))?;
    let mut receiver = build_expression(first)?;

    for suffix_pair in inner {
        let suffix_inner = suffix_pair
            .into_inner()
            .next()
            .ok_or_else(|| CccError::parse("expected postfix suffix".to_string()))?;
        match suffix_inner.as_rule() {
            Rule::method_call => {
                receiver = build_method_call(receiver, suffix_inner)?;
            }
            Rule::type_cast => {
                receiver = build_type_cast(receiver, suffix_inner)?;
            }
            _ => {
                return Err(CccError::parse(format!(
                    "unexpected postfix rule: {:?}",
                    suffix_inner.as_rule()
                )));
            }
        }
    }

    Ok(receiver)
}

/// Desugar `receiver.method(args)` into `method(receiver, args)`.
fn build_method_call(
    receiver: Expression,
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let mut method_inner = pair.into_inner();
    let name = method_inner
        .next()
        .ok_or_else(|| CccError::parse("expected method name".to_string()))?
        .as_str()
        .to_string();
    let mut arguments = vec![receiver];
    if let Some(args_pair) = method_inner.next() {
        for arg in args_pair.into_inner() {
            arguments.push(build_expression(arg)?);
        }
    }
    Ok(Expression::FunctionCall { name, arguments })
}

fn build_type_cast(
    operand: Expression,
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let cast_type_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| CccError::parse("expected cast target type".to_string()))?;
    let target_type = match cast_type_pair.as_str() {
        "int" => CastTargetType::Integer,
        "float" => CastTargetType::Float,
        "timestamp" => CastTargetType::Timestamp,
        "datetime" => CastTargetType::DateTime,
        other => {
            return Err(CccError::parse(format!(
                "unsupported cast target type: {other}"
            )));
        }
    };
    Ok(Expression::TypeCast {
        operand: Box::new(operand),
        target_type,
    })
}
