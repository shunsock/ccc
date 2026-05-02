mod builtin_constructor;
mod builtin_helpers;
mod builtin_list;
mod builtin_math;
mod builtin_time;

use domain::error::CccError;
use domain::value::Value;

/// Dispatch a builtin function call.
pub fn call_builtin(name: &str, arguments: &[Value]) -> Result<Value, CccError> {
    match name {
        // Math
        "sqrt" => builtin_math::unary_float_function(name, arguments, f64::sqrt),
        "abs" => builtin_math::unary_absolute(arguments),
        "sin" => builtin_math::unary_float_function(name, arguments, f64::sin),
        "cos" => builtin_math::unary_float_function(name, arguments, f64::cos),
        "tan" => builtin_math::unary_float_function(name, arguments, f64::tan),
        "arcsin" => builtin_math::unary_float_function(name, arguments, f64::asin),
        "arccos" => builtin_math::unary_float_function(name, arguments, f64::acos),
        "arctan" => builtin_math::unary_float_function(name, arguments, f64::atan),
        "log" => builtin_math::unary_float_function(name, arguments, f64::ln),
        "log2" => builtin_math::unary_float_function(name, arguments, f64::log2),
        "log10" => builtin_math::unary_float_function(name, arguments, f64::log10),
        "floor" => builtin_math::unary_float_function(name, arguments, f64::floor),
        "ceil" => builtin_math::unary_float_function(name, arguments, f64::ceil),
        "round" => builtin_math::unary_float_function(name, arguments, f64::round),

        // List & Statistics
        "len" => builtin_list::list_len(arguments),
        "sum" => builtin_list::list_sum(arguments),
        "prod" => builtin_list::list_prod(arguments),
        "mean" => builtin_list::list_mean(arguments),
        "variance" => builtin_list::list_variance(arguments),
        "max" => builtin_list::list_extremum("max", arguments, i64::max, f64::max, i64::max),
        "min" => builtin_list::list_extremum("min", arguments, i64::min, f64::min, i64::min),
        "median" => builtin_list::list_median(arguments),
        "head" => builtin_list::list_head(arguments),
        "tail" => builtin_list::list_tail(arguments),

        // Constructors
        "DurationTime" => builtin_constructor::duration_time_constructor(arguments),
        "DateTime" => builtin_constructor::datetime_constructor(arguments),
        "Timestamp" => builtin_constructor::timestamp_constructor(arguments),

        // Time utilities
        "now" => builtin_time::now_function(arguments),
        "today" => builtin_time::today_function(arguments),
        "current_timestamp" => builtin_time::current_timestamp_function(arguments),

        _ => Err(CccError::eval(format!("unknown function: {name}"))),
    }
}
