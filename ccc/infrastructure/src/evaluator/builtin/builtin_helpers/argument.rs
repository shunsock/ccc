use domain::error::CccError;
use domain::value::Value;

pub fn expect_single_arg<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a Value, CccError> {
    if arguments.len() != 1 {
        return Err(CccError::eval(format!(
            "{name} expects 1 argument, got {}",
            arguments.len()
        )));
    }
    Ok(&arguments[0])
}

pub fn expect_no_args(name: &str, arguments: &[Value]) -> Result<(), CccError> {
    if !arguments.is_empty() {
        return Err(CccError::eval(format!(
            "{name} expects 0 arguments, got {}",
            arguments.len()
        )));
    }
    Ok(())
}

pub fn expect_single_list<'a>(name: &str, arguments: &'a [Value]) -> Result<&'a [Value], CccError> {
    let arg = expect_single_arg(name, arguments)?;
    match arg {
        Value::List(elements) => Ok(elements.as_slice()),
        _ => Err(CccError::eval(format!("{name}: expected list"))),
    }
}

pub fn expect_nonempty_list<'a>(
    name: &str,
    arguments: &'a [Value],
) -> Result<&'a [Value], CccError> {
    let elements = expect_single_list(name, arguments)?;
    if elements.is_empty() {
        return Err(CccError::eval(format!("{name}: empty list")));
    }
    Ok(elements)
}
