mod argument_rule;
mod ast_type_checker;
mod binary_rule;
mod cast_rule;
mod function_rule;

pub use ast_type_checker::AstTypeChecker;

#[cfg(test)]
mod ast_type_checker_test;
