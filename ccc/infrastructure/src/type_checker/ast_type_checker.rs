use domain::ast::{AbstractSyntaxTree, Expression};
use domain::error::CccError;
use domain::interface::type_checker::CccTypeChecker;
use domain::static_type::StaticType;

use super::binary_rule::infer_binary_type;
use super::cast_rule::infer_cast_type;
use super::function_rule::infer_function_return_type;

pub struct AstTypeChecker;

impl CccTypeChecker for AstTypeChecker {
    fn check(&self, ast: &AbstractSyntaxTree) -> Result<(), CccError> {
        infer_type(&ast.expression)?;
        Ok(())
    }
}

/// Infer the static type of an expression, returning an error for invalid combinations.
pub(super) fn infer_type(expression: &Expression) -> Result<StaticType, CccError> {
    match expression {
        Expression::Integer(_) => Ok(StaticType::Integer),
        Expression::Float(_) => Ok(StaticType::Float),
        Expression::List(elements) => {
            let elem_type = infer_list_element_type(elements)?;
            Ok(StaticType::List(elem_type.map(Box::new)))
        }
        Expression::DurationTime { .. } => Ok(StaticType::DurationTime),
        Expression::DateTime { .. } => Ok(StaticType::DateTime),

        Expression::UnaryOperation {
            operator: _,
            operand,
        } => {
            let operand_type = infer_type(operand)?;
            match operand_type {
                StaticType::Integer | StaticType::Float | StaticType::DurationTime => {
                    Ok(operand_type)
                }
                _ => Err(CccError::type_check(format!(
                    "cannot apply unary operator to {operand_type}"
                ))),
            }
        }

        Expression::BinaryOperation {
            operator,
            left,
            right,
        } => {
            let left_type = infer_type(left)?;
            let right_type = infer_type(right)?;
            infer_binary_type(operator, &left_type, &right_type)
        }

        Expression::TypeCast {
            operand,
            target_type,
        } => {
            let operand_type = infer_type(operand)?;
            infer_cast_type(&operand_type, target_type)
        }

        Expression::FunctionCall { name, arguments } => {
            // Infer argument types (to validate nested expressions)
            let arg_types: Vec<StaticType> =
                arguments.iter().map(infer_type).collect::<Result<_, _>>()?;
            infer_function_return_type(name, &arg_types)
        }
    }
}

/// Validate that all elements of a list share the same type and return it.
/// Returns `None` for empty lists.
fn infer_list_element_type(elements: &[Expression]) -> Result<Option<StaticType>, CccError> {
    let first = match elements.first() {
        None => return Ok(None),
        Some(e) => e,
    };
    let expected = infer_type(first)?;
    for (i, elem) in elements.iter().enumerate().skip(1) {
        let actual = infer_type(elem)?;
        if actual != expected {
            return Err(CccError::type_check(format!(
                "list elements must be the same type, expected {expected} at index 0 but found {actual} at index {}",
                i
            )));
        }
    }
    Ok(Some(expected))
}
