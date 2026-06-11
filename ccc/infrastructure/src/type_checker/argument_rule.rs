use domain::error::CccError;
use domain::static_type::StaticType;

pub(super) fn check_arg_count(
    name: &str,
    args: &[StaticType],
    expected: usize,
) -> Result<(), CccError> {
    if args.len() != expected {
        return Err(CccError::type_check(format!(
            "{name} expects {expected} argument(s), got {}",
            args.len()
        )));
    }
    Ok(())
}

pub(super) fn require_numeric(name: &str, t: &StaticType) -> Result<(), CccError> {
    match t {
        StaticType::Integer | StaticType::Float | StaticType::Unknown => Ok(()),
        _ => Err(CccError::type_check(format!(
            "{name}: expected numeric argument, got {t}"
        ))),
    }
}

pub(super) fn require_list(name: &str, actual: &StaticType) -> Result<(), CccError> {
    match actual {
        StaticType::List(_) | StaticType::Unknown => Ok(()),
        _ => Err(CccError::type_check(format!(
            "{name}: expected list, got {actual}"
        ))),
    }
}

pub(super) fn require_type_at(
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
