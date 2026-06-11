use domain::ast::Expression;
use domain::error::CccError;

use super::operator_mapping::to_unary_operation;
use super::pest_based_parser::{Rule, build_expression};

/// Build a unary expression, collapsing stacked operators like `--3`.
pub(super) fn build_unary(pair: pest::iterators::Pair<Rule>) -> Result<Expression, CccError> {
    let mut inner = pair.into_inner();

    // Collect unary operators (zero or more)
    let mut operators = Vec::new();
    while let Some(pair) = inner.peek() {
        if pair.as_rule() != Rule::unary_operator {
            break;
        }
        let op_pair = inner.next().expect("peeked pair must exist");
        operators.push(to_unary_operation(&op_pair));
    }

    let operand_pair = inner
        .next()
        .ok_or_else(|| CccError::parse("expected expression".to_string()))?;
    let mut result = build_expression(operand_pair)?;

    // Wrap from innermost to outermost
    for operator in operators.into_iter().rev() {
        result = Expression::UnaryOperation {
            operator,
            operand: Box::new(result),
        };
    }

    Ok(result)
}
