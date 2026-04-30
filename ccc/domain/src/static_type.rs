/// Static type representation used during type checking.
#[derive(Debug, Clone, PartialEq)]
pub enum StaticType {
    Integer,
    Float,
    /// List with a known element type. `None` represents an empty list.
    List(Option<Box<StaticType>>),
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
            StaticType::List(None) => write!(f, "list"),
            StaticType::List(Some(elem)) => write!(f, "list[{elem}]"),
            StaticType::DurationTime => write!(f, "duration"),
            StaticType::DateTime => write!(f, "datetime"),
            StaticType::Timestamp => write!(f, "timestamp"),
            StaticType::Unknown => write!(f, "unknown"),
        }
    }
}
