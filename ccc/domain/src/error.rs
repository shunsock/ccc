use thiserror::Error;

/// Position in the input where an error occurred.
#[derive(Debug, Clone, PartialEq)]
pub struct SourcePosition {
    /// 1-based column number.
    pub column: usize,
}

#[derive(Debug, Error, PartialEq)]
pub enum CccError {
    #[error("scan error: {message}")]
    Scan { message: String },

    #[error("parse error: {message}")]
    Parse {
        message: String,
        position: Option<SourcePosition>,
    },

    #[error("eval error: {message}")]
    Eval { message: String },

    #[error("type error: {message}")]
    TypeCheck {
        message: String,
        position: Option<SourcePosition>,
    },
}

impl CccError {
    pub fn scan(message: impl Into<String>) -> Self {
        Self::Scan {
            message: message.into(),
        }
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
            position: None,
        }
    }

    pub fn parse_at(message: impl Into<String>, column: usize) -> Self {
        Self::Parse {
            message: message.into(),
            position: Some(SourcePosition { column }),
        }
    }

    pub fn eval(message: impl Into<String>) -> Self {
        Self::Eval {
            message: message.into(),
        }
    }

    pub fn type_check(message: impl Into<String>) -> Self {
        Self::TypeCheck {
            message: message.into(),
            position: None,
        }
    }

    pub fn type_check_at(message: impl Into<String>, column: usize) -> Self {
        Self::TypeCheck {
            message: message.into(),
            position: Some(SourcePosition { column }),
        }
    }
}
