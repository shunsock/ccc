use domain::ast::{AbstractSyntaxTree, BinaryOperation, CastTargetType, Expression};
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

/// Determine the result type of a type cast, or error if unsupported.
fn infer_cast_type(
    operand_type: &StaticType,
    target_type: &CastTargetType,
) -> Result<StaticType, CccError> {
    match (operand_type, target_type) {
        // Numeric casts
        (StaticType::Integer | StaticType::Float, CastTargetType::Integer) => {
            Ok(StaticType::Integer)
        }
        (StaticType::Integer | StaticType::Float, CastTargetType::Float) => Ok(StaticType::Float),
        // DateTime → Timestamp
        (StaticType::DateTime, CastTargetType::Timestamp) => Ok(StaticType::Timestamp),
        // Timestamp → DateTime
        (StaticType::Timestamp, CastTargetType::DateTime) => Ok(StaticType::DateTime),
        // Unknown passes through
        (StaticType::Unknown, target) => match target {
            CastTargetType::Integer => Ok(StaticType::Integer),
            CastTargetType::Float => Ok(StaticType::Float),
            CastTargetType::Timestamp => Ok(StaticType::Timestamp),
            CastTargetType::DateTime => Ok(StaticType::DateTime),
        },
        _ => {
            let target_name = match target_type {
                CastTargetType::Integer => "int",
                CastTargetType::Float => "float",
                CastTargetType::Timestamp => "timestamp",
                CastTargetType::DateTime => "datetime",
            };
            Err(CccError::type_check(format!(
                "cannot cast {operand_type} to {target_name}"
            )))
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
            require_list(name, &arg_types[0])?;
            Ok(StaticType::Integer)
        }
        "sum" | "prod" | "mean" | "variance" | "max" | "min" | "median" => {
            check_arg_count(name, arg_types, 1)?;
            require_list(name, &arg_types[0])?;
            Ok(StaticType::Unknown)
        }
        "head" | "tail" => {
            check_arg_count(name, arg_types, 1)?;
            require_list(name, &arg_types[0])?;
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

fn require_list(name: &str, actual: &StaticType) -> Result<(), CccError> {
    match actual {
        StaticType::List(_) | StaticType::Unknown => Ok(()),
        _ => Err(CccError::type_check(format!(
            "{name}: expected list, got {actual}"
        ))),
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
