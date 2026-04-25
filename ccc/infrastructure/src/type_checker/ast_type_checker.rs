use domain::ast::{AbstractSyntaxTree, BinaryOperation, Expression};
use domain::error::CccError;
use domain::interface::type_checker::CccTypeChecker;
use domain::static_type::StaticType;

pub struct AstTypeChecker;

impl CccTypeChecker for AstTypeChecker {
    fn check(&self, ast: &AbstractSyntaxTree) -> Result<(), CccError> {
        infer_type(&ast.expression)?;
        Ok(())
    }
}

/// Infer the static type of an expression, returning an error for invalid combinations.
fn infer_type(expression: &Expression) -> Result<StaticType, CccError> {
    match expression {
        Expression::Integer(_) => Ok(StaticType::Integer),
        Expression::Float(_) => Ok(StaticType::Float),
        Expression::List(_) => Ok(StaticType::List),
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

        Expression::FunctionCall { name, arguments } => {
            // Infer argument types (to validate nested expressions)
            let arg_types: Vec<StaticType> =
                arguments.iter().map(infer_type).collect::<Result<_, _>>()?;
            infer_function_return_type(name, &arg_types)
        }
    }
}

/// Determine the result type of a binary operation, or error if unsupported.
fn infer_binary_type(
    operator: &BinaryOperation,
    left: &StaticType,
    right: &StaticType,
) -> Result<StaticType, CccError> {
    use BinaryOperation::*;
    use StaticType::*;

    match (left, operator, right) {
        // Numeric: all operators allowed
        (Integer, _, Integer) => Ok(Integer),
        (Integer, _, Float) | (Float, _, Integer) | (Float, _, Float) => Ok(Float),

        // DurationTime ± DurationTime → DurationTime
        (DurationTime, Add | Subtract, DurationTime) => Ok(DurationTime),
        // DurationTime * Integer, DurationTime / Integer → DurationTime
        (DurationTime, Multiply | Divide, Integer) => Ok(DurationTime),
        // Integer * DurationTime → DurationTime
        (Integer, Multiply, DurationTime) => Ok(DurationTime),

        // DateTime ± DurationTime → DateTime
        (DateTime, Add | Subtract, DurationTime) => Ok(DateTime),
        // DateTime - DateTime → DurationTime
        (DateTime, Subtract, DateTime) => Ok(DurationTime),

        // Timestamp ± DurationTime → Timestamp
        (Timestamp, Add | Subtract, DurationTime) => Ok(Timestamp),
        // Timestamp - Timestamp → DurationTime
        (Timestamp, Subtract, Timestamp) => Ok(DurationTime),

        // Unknown passes through (skeleton compatibility)
        (Unknown, _, _) | (_, _, Unknown) => Ok(Unknown),

        _ => {
            let op_str = operator_str(operator);
            Err(CccError::type_check(format!(
                "unsupported operation: {left} {op_str} {right}"
            )))
        }
    }
}

fn operator_str(operator: &BinaryOperation) -> &'static str {
    match operator {
        BinaryOperation::Add => "+",
        BinaryOperation::Subtract => "-",
        BinaryOperation::Multiply => "*",
        BinaryOperation::Divide => "/",
        BinaryOperation::Modulo => "%",
        BinaryOperation::Power => "^",
    }
}

/// Infer the return type of a builtin function call.
fn infer_function_return_type(
    name: &str,
    arg_types: &[StaticType],
) -> Result<StaticType, CccError> {
    match name {
        // Math functions: require numeric input, return Float
        "sqrt" | "sin" | "cos" | "tan" | "arcsin" | "arccos" | "arctan" | "log" | "log2"
        | "log10" | "floor" | "ceil" | "round" => {
            check_arg_count(name, arg_types, 1)?;
            require_numeric(name, &arg_types[0])?;
            Ok(StaticType::Float)
        }
        "abs" => {
            check_arg_count(name, arg_types, 1)?;
            require_numeric(name, &arg_types[0])?;
            Ok(arg_types[0].clone())
        }

        // List functions
        "len" => {
            check_arg_count(name, arg_types, 1)?;
            require_type(name, &arg_types[0], &StaticType::List)?;
            Ok(StaticType::Integer)
        }
        "sum" | "prod" => {
            check_arg_count(name, arg_types, 1)?;
            require_type(name, &arg_types[0], &StaticType::List)?;
            Ok(StaticType::Unknown) // element type unknown at static level
        }
        "head" | "tail" => {
            check_arg_count(name, arg_types, 1)?;
            require_type(name, &arg_types[0], &StaticType::List)?;
            Ok(StaticType::Unknown)
        }

        // Time constructors
        "DurationTime" => {
            if arg_types.len() < 3 || arg_types.len() > 4 {
                return Err(CccError::type_check(format!(
                    "DurationTime expects 3 or 4 arguments, got {}",
                    arg_types.len()
                )));
            }
            for (i, t) in arg_types.iter().enumerate() {
                require_type_at(name, t, &StaticType::Integer, i)?;
            }
            Ok(StaticType::DurationTime)
        }
        "DateTime" => {
            check_arg_count(name, arg_types, 6)?;
            for (i, t) in arg_types.iter().enumerate() {
                require_type_at(name, t, &StaticType::Integer, i)?;
            }
            Ok(StaticType::DateTime)
        }
        "Timestamp" => {
            check_arg_count(name, arg_types, 1)?;
            require_numeric(name, &arg_types[0])?;
            Ok(StaticType::Timestamp)
        }

        // Conversion functions
        "datetime_to_timestamp" => {
            check_arg_count(name, arg_types, 1)?;
            require_type(name, &arg_types[0], &StaticType::DateTime)?;
            Ok(StaticType::Timestamp)
        }
        "timestamp_to_datetime" => {
            if arg_types.is_empty() || arg_types.len() > 2 {
                return Err(CccError::type_check(format!(
                    "timestamp_to_datetime expects 1-2 arguments, got {}",
                    arg_types.len()
                )));
            }
            require_type(name, &arg_types[0], &StaticType::Timestamp)?;
            if arg_types.len() == 2 {
                require_type_at(name, &arg_types[1], &StaticType::Integer, 1)?;
            }
            Ok(StaticType::DateTime)
        }

        // Time utility functions (zero arguments)
        "now" | "today" => {
            check_arg_count(name, arg_types, 0)?;
            Ok(StaticType::DateTime)
        }
        "current_timestamp" => {
            check_arg_count(name, arg_types, 0)?;
            Ok(StaticType::Timestamp)
        }

        _ => Err(CccError::type_check(format!("undefined function: {name}"))),
    }
}

fn check_arg_count(name: &str, args: &[StaticType], expected: usize) -> Result<(), CccError> {
    if args.len() != expected {
        return Err(CccError::type_check(format!(
            "{name} expects {expected} argument(s), got {}",
            args.len()
        )));
    }
    Ok(())
}

fn require_numeric(name: &str, t: &StaticType) -> Result<(), CccError> {
    match t {
        StaticType::Integer | StaticType::Float | StaticType::Unknown => Ok(()),
        _ => Err(CccError::type_check(format!(
            "{name}: expected numeric argument, got {t}"
        ))),
    }
}

fn require_type(name: &str, actual: &StaticType, expected: &StaticType) -> Result<(), CccError> {
    if actual == expected || *actual == StaticType::Unknown {
        Ok(())
    } else {
        Err(CccError::type_check(format!(
            "{name}: expected {expected}, got {actual}"
        )))
    }
}

fn require_type_at(
    name: &str,
    actual: &StaticType,
    expected: &StaticType,
    index: usize,
) -> Result<(), CccError> {
    if actual == expected || *actual == StaticType::Unknown {
        Ok(())
    } else {
        Err(CccError::type_check(format!(
            "{name}: argument {} expected {expected}, got {actual}",
            index + 1
        )))
    }
}
