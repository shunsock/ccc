use domain::ast::{AbstractSyntaxTree, Expression};
use domain::error::CccError;
use domain::interface::evaluator::CccEvaluator;
use domain::value::Value;

use super::binary_operation::evaluate_binary;
use super::builtin;
use super::type_cast::evaluate_type_cast;
use super::unary_operation::evaluate_unary;

pub struct AstEvaluator;

impl CccEvaluator for AstEvaluator {
    fn evaluate(&self, ast: &AbstractSyntaxTree) -> Result<Value, CccError> {
        evaluate_expression(&ast.expression)
    }
}

pub(super) fn evaluate_expression(expression: &Expression) -> Result<Value, CccError> {
    match expression {
        Expression::Integer(n) => Ok(Value::Integer(*n)),
        Expression::Float(n) => Ok(Value::Float(*n)),
        Expression::BinaryOperation {
            operator,
            left,
            right,
        } => {
            let left_value = evaluate_expression(left)?;
            let right_value = evaluate_expression(right)?;
            evaluate_binary(operator, &left_value, &right_value)
        }
        Expression::UnaryOperation { operator, operand } => {
            let value = evaluate_expression(operand)?;
            evaluate_unary(operator, &value)
        }
        Expression::FunctionCall { name, arguments } => {
            let evaluated_arguments: Result<Vec<Value>, CccError> =
                arguments.iter().map(evaluate_expression).collect();
            builtin::call_builtin(name, &evaluated_arguments?)
        }
        Expression::TypeCast {
            operand,
            target_type,
        } => {
            let value = evaluate_expression(operand)?;
            evaluate_type_cast(&value, target_type)
        }
        Expression::List(elements) => {
            let evaluated: Result<Vec<Value>, CccError> =
                elements.iter().map(evaluate_expression).collect();
            Ok(Value::List(evaluated?))
        }
        Expression::DurationTime {
            hours,
            minutes,
            seconds,
        } => {
            let total_seconds = (*hours) * 3600 + (*minutes as i64) * 60 + (*seconds as i64);
            Ok(Value::DurationTime(total_seconds))
        }
        Expression::DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_seconds,
        } => {
            let local_epoch = domain::calendar::calendar_to_epoch_seconds(
                *year, *month, *day, *hour, *minute, *second,
            )
            .ok_or_else(|| CccError::eval("invalid datetime components"))?;
            // Convert local time to UTC by subtracting the offset
            let utc_epoch = local_epoch - (*offset_seconds as i64);
            Ok(Value::DateTime {
                epoch_seconds: utc_epoch,
                offset_seconds: *offset_seconds,
            })
        }
    }
}
