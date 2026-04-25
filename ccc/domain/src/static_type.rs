/// Static type representation used during type checking.
#[derive(Debug, Clone, PartialEq)]
pub enum StaticType {
    Integer,
    Float,
    List,
    DurationTime,
    DateTime,
    Timestamp,
    Unknown,
}

impl std::fmt::Display for StaticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StaticType::Integer => write!(f, "integer"),
            StaticType::Float => write!(f, "float"),
            StaticType::List => write!(f, "list"),
            StaticType::DurationTime => write!(f, "duration"),
            StaticType::DateTime => write!(f, "datetime"),
            StaticType::Timestamp => write!(f, "timestamp"),
            StaticType::Unknown => write!(f, "unknown"),
        }
    }
}
