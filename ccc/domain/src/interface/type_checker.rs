use crate::ast::AbstractSyntaxTree;
use crate::error::CccError;

/// Interface for static type checking before evaluation.
pub trait CccTypeChecker {
    fn check(&self, ast: &AbstractSyntaxTree) -> Result<(), CccError>;
}
