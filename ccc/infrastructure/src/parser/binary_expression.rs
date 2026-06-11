use domain::ast::{BinaryOperation, Expression};
use domain::error::CccError;

use super::pest_based_parser::{Rule, build_expression};

/// Build a left-associative binary expression chain.
/// `1 - 2 - 3` becomes `(1 - 2) - 3`.
pub(super) fn build_binary_expression(
    pair: pest::iterators::Pair<Rule>,
    operation_mapper: fn(&pest::iterators::Pair<Rule>) -> BinaryOperation,
) -> Result<Expression, CccError> {
    let mut inner = pair.into_inner();
    let first = inner
        .next()
        .ok_or_else(|| CccError::parse("expected operand".to_string()))?;
    let mut left = build_expression(first)?;

    while let Some(operator_pair) = inner.next() {
        let right_pair = inner
            .next()
            .ok_or_else(|| CccError::parse("expected right operand".to_string()))?;
        let operator = operation_mapper(&operator_pair);
        let right = build_expression(right_pair)?;
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
pub(super) fn build_right_associative_expression(
    pair: pest::iterators::Pair<Rule>,
) -> Result<Expression, CccError> {
    let mut operands = Vec::new();
    let mut inner = pair.into_inner();

    let first = inner
        .next()
        .ok_or_else(|| CccError::parse("expected operand".to_string()))?;
    operands.push(build_expression(first)?);

    while let Some(_operator_pair) = inner.next() {
        let right_pair = inner
            .next()
            .ok_or_else(|| CccError::parse("expected right operand".to_string()))?;
        operands.push(build_expression(right_pair)?);
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
