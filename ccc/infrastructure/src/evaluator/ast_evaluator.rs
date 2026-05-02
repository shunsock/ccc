use domain::ast::{
    AbstractSyntaxTree, BinaryOperation, CastTargetType, Expression, UnaryOperation,
};
use domain::error::CccError;
use domain::interface::evaluator::CccEvaluator;
use domain::value::Value;

use super::builtin;

pub struct AstEvaluator;

impl CccEvaluator for AstEvaluator {
    fn evaluate(&self, ast: &AbstractSyntaxTree) -> Result<Value, CccError> {
        evaluate_expression(&ast.expression)
    }
}

fn evaluate_expression(expression: &Expression) -> Result<Value, CccError> {
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
            let local_epoch = domain::value::calendar_to_epoch_seconds(
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

fn evaluate_binary(
    operator: &BinaryOperation,
    left: &Value,
    right: &Value,
) -> Result<Value, CccError> {
    match (left, right) {
        // Numeric operations
        (Value::Integer(l), Value::Integer(r)) => evaluate_binary_integer(operator, *l, *r),
        (Value::Integer(_) | Value::Float(_), Value::Integer(_) | Value::Float(_)) => {
            evaluate_binary_float(operator, to_f64(left)?, to_f64(right)?)
        }

        // DurationTime ± DurationTime → DurationTime
        (Value::DurationTime(l), Value::DurationTime(r)) => match operator {
            BinaryOperation::Add => Ok(Value::DurationTime(l + r)),
            BinaryOperation::Subtract => Ok(Value::DurationTime(l - r)),
            _ => Err(unsupported_binary_op(operator, "duration", "duration")),
        },
        // DurationTime * Integer → DurationTime
        (Value::DurationTime(d), Value::Integer(n)) => match operator {
            BinaryOperation::Multiply => Ok(Value::DurationTime(d * n)),
            BinaryOperation::Divide => {
                if *n == 0 {
                    return Err(CccError::eval("division by zero"));
                }
                Ok(Value::DurationTime(d / n))
            }
            _ => Err(unsupported_binary_op(operator, "duration", "integer")),
        },
        // Integer * DurationTime → DurationTime
        (Value::Integer(n), Value::DurationTime(d)) => match operator {
            BinaryOperation::Multiply => Ok(Value::DurationTime(n * d)),
            _ => Err(unsupported_binary_op(operator, "integer", "duration")),
        },

        // DateTime ± DurationTime → DateTime
        (
            Value::DateTime {
                epoch_seconds,
                offset_seconds,
            },
            Value::DurationTime(d),
        ) => match operator {
            BinaryOperation::Add => Ok(Value::DateTime {
                epoch_seconds: epoch_seconds + d,
                offset_seconds: *offset_seconds,
            }),
            BinaryOperation::Subtract => Ok(Value::DateTime {
                epoch_seconds: epoch_seconds - d,
                offset_seconds: *offset_seconds,
            }),
            _ => Err(unsupported_binary_op(operator, "datetime", "duration")),
        },
        // DateTime - DateTime → DurationTime
        (
            Value::DateTime {
                epoch_seconds: l, ..
            },
            Value::DateTime {
                epoch_seconds: r, ..
            },
        ) => match operator {
            BinaryOperation::Subtract => Ok(Value::DurationTime(l - r)),
            _ => Err(unsupported_binary_op(operator, "datetime", "datetime")),
        },

        // Timestamp ± DurationTime → Timestamp
        (Value::Timestamp(ts), Value::DurationTime(d)) => match operator {
            BinaryOperation::Add => Ok(Value::Timestamp(ts + *d as f64)),
            BinaryOperation::Subtract => Ok(Value::Timestamp(ts - *d as f64)),
            _ => Err(unsupported_binary_op(operator, "timestamp", "duration")),
        },
        // Timestamp - Timestamp → DurationTime
        (Value::Timestamp(l), Value::Timestamp(r)) => match operator {
            BinaryOperation::Subtract => Ok(Value::DurationTime((l - r) as i64)),
            _ => Err(unsupported_binary_op(operator, "timestamp", "timestamp")),
        },

        _ => Err(unsupported_binary_op(
            operator,
            &type_name(left),
            &type_name(right),
        )),
    }
}

fn unsupported_binary_op(operator: &BinaryOperation, left: &str, right: &str) -> CccError {
    let op_str = match operator {
        BinaryOperation::Add => "+",
        BinaryOperation::Subtract => "-",
        BinaryOperation::Multiply => "*",
        BinaryOperation::Divide => "/",
        BinaryOperation::Modulo => "%",
        BinaryOperation::Power => "^",
    };
    CccError::eval(format!("unsupported operation: {left} {op_str} {right}"))
}

fn type_name(value: &Value) -> String {
    match value {
        Value::Integer(_) => "integer".to_string(),
        Value::Float(_) => "float".to_string(),
        Value::List(_) => "list".to_string(),
        Value::DurationTime(_) => "duration".to_string(),
        Value::DateTime { .. } => "datetime".to_string(),
        Value::Timestamp(_) => "timestamp".to_string(),
    }
}

fn evaluate_binary_integer(
    operator: &BinaryOperation,
    left: i64,
    right: i64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::Integer(left + right)),
        BinaryOperation::Subtract => Ok(Value::Integer(left - right)),
        BinaryOperation::Multiply => Ok(Value::Integer(left * right)),
        BinaryOperation::Divide => {
            if right == 0 {
                return Err(CccError::eval(format!(
                    "division by zero: {left} / {right}"
                )));
            }
            if left % right == 0 {
                Ok(Value::Integer(left / right))
            } else {
                Ok(Value::Float(left as f64 / right as f64))
            }
        }
        BinaryOperation::Modulo => {
            if right == 0 {
                return Err(CccError::eval(format!("modulo by zero: {left} % {right}")));
            }
            Ok(Value::Integer(left % right))
        }
        BinaryOperation::Power => {
            if right >= 0 && right <= u32::MAX as i64 {
                Ok(Value::Integer(left.pow(right as u32)))
            } else {
                Ok(Value::Float((left as f64).powf(right as f64)))
            }
        }
    }
}

fn evaluate_binary_float(
    operator: &BinaryOperation,
    left: f64,
    right: f64,
) -> Result<Value, CccError> {
    match operator {
        BinaryOperation::Add => Ok(Value::Float(left + right)),
        BinaryOperation::Subtract => Ok(Value::Float(left - right)),
        BinaryOperation::Multiply => Ok(Value::Float(left * right)),
        BinaryOperation::Divide => {
            if right == 0.0 {
                return Err(CccError::eval(format!(
                    "division by zero: {left} / {right}"
                )));
            }
            Ok(Value::Float(left / right))
        }
        BinaryOperation::Modulo => {
            if right == 0.0 {
                return Err(CccError::eval(format!("modulo by zero: {left} % {right}")));
            }
            Ok(Value::Float(left % right))
        }
        BinaryOperation::Power => Ok(Value::Float(left.powf(right))),
    }
}

fn evaluate_unary(operator: &UnaryOperation, value: &Value) -> Result<Value, CccError> {
    match (operator, value) {
        (UnaryOperation::Negate, Value::Integer(n)) => n
            .checked_neg()
            .map(Value::Integer)
            .ok_or_else(|| CccError::eval("integer negation overflow".to_string())),
        (UnaryOperation::Negate, Value::Float(n)) => Ok(Value::Float(-n)),
        (UnaryOperation::Negate, Value::List(_)) => Err(CccError::eval("cannot negate a list")),
        (UnaryOperation::Negate, Value::DurationTime(s)) => Ok(Value::DurationTime(-s)),
        (UnaryOperation::Negate, Value::DateTime { .. }) => {
            Err(CccError::eval("cannot negate a datetime"))
        }
        (UnaryOperation::Negate, Value::Timestamp(_)) => {
            Err(CccError::eval("cannot negate a timestamp"))
        }
        (UnaryOperation::Positive, v) => Ok(v.clone()),
    }
}

fn evaluate_type_cast(value: &Value, target_type: &CastTargetType) -> Result<Value, CccError> {
    match target_type {
        CastTargetType::Integer => {
            let n = to_i64(value)?;
            Ok(Value::Integer(n))
        }
        CastTargetType::Float => {
            let n = to_f64(value)?;
            Ok(Value::Float(n))
        }
        CastTargetType::Timestamp => match value {
            Value::DateTime { epoch_seconds, .. } => Ok(Value::Timestamp(*epoch_seconds as f64)),
            _ => Err(CccError::eval(format!(
                "cannot cast {} to timestamp",
                type_name(value)
            ))),
        },
        CastTargetType::DateTime => match value {
            Value::Timestamp(ts) => Ok(Value::DateTime {
                epoch_seconds: *ts as i64,
                offset_seconds: 0,
            }),
            _ => Err(CccError::eval(format!(
                "cannot cast {} to datetime",
                type_name(value)
            ))),
        },
    }
}

fn to_f64(value: &Value) -> Result<f64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Float(n) => Ok(*n),
        Value::List(_) => Err(CccError::eval("expected number, got list")),
        Value::DurationTime(_) => Err(CccError::eval("expected number, got duration")),
        Value::DateTime { .. } => Err(CccError::eval("expected number, got datetime")),
        Value::Timestamp(_) => Err(CccError::eval("expected number, got timestamp")),
    }
}

fn to_i64(value: &Value) -> Result<i64, CccError> {
    match value {
        Value::Integer(n) => Ok(*n),
        Value::Float(n) => Ok(*n as i64),
        Value::List(_) => Err(CccError::eval("expected number, got list")),
        Value::DurationTime(_) => Err(CccError::eval("expected number, got duration")),
        Value::DateTime { .. } => Err(CccError::eval("expected number, got datetime")),
        Value::Timestamp(_) => Err(CccError::eval("expected number, got timestamp")),
    }
}
