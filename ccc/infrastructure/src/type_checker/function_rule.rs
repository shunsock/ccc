use domain::error::CccError;
use domain::static_type::StaticType;

use super::argument_rule::{check_arg_count, require_list, require_numeric, require_type_at};

/// Infer the return type of a builtin function call.
pub(super) fn infer_function_return_type(
    name: &str,
    arg_types: &[StaticType],
) -> Result<StaticType, CccError> {
    match name {
        // Math functions: require numeric input, return Float
        "sqrt" | "sin" | "cos" | "tan" | "arcsin" | "arccos" | "arctan" | "ln" | "floor"
        | "ceil" | "round" => {
            check_arg_count(name, arg_types, 1)?;
            require_numeric(name, &arg_types[0])?;
            Ok(StaticType::Float)
        }
        // log: 1 arg (natural log) or 2 args (log with base)
        "log" => {
            if arg_types.is_empty() || arg_types.len() > 2 {
                return Err(CccError::type_check(format!(
                    "log expects 1 or 2 arguments, got {}",
                    arg_types.len()
                )));
            }
            for arg in arg_types {
                require_numeric(name, arg)?;
            }
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
        "sum" | "prod" | "mean" | "variance" | "max" | "min" | "median" | "head" | "tail" => {
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
