use crate::ast::AbstractSyntaxTree;
use crate::error::CccError;

/// Interface for syntactic analysis.
pub trait CccParser {
    fn parse(&self, input: &str) -> Result<AbstractSyntaxTree, CccError>;
}
