use std::io::{self, BufRead, Write};

use domain::interface::evaluator::CccEvaluator;
use domain::interface::parser::CccParser;
use domain::interface::type_checker::CccTypeChecker;

use crate::calculate_math_expression::CalculateMathExpressionUsecase;
use crate::format_error::format_error_with_caret;

/// Result of evaluating piped input lines.
pub struct PipedEvaluationResult {
    pub has_error: bool,
    pub had_input: bool,
}

/// Usecase: evaluate expressions read from a buffered reader line by line.
pub struct EvaluatePipedInputUsecase<P, T, E> {
    calculate: CalculateMathExpressionUsecase<P, T, E>,
}

impl<P, T, E> EvaluatePipedInputUsecase<P, T, E>
where
    P: CccParser,
    T: CccTypeChecker,
    E: CccEvaluator,
{
    pub fn new(calculate: CalculateMathExpressionUsecase<P, T, E>) -> Self {
        Self { calculate }
    }

    /// Evaluate each line from the reader. If `args_suffix` is provided,
    /// append it to each stdin line before evaluation.
    pub fn run(
        &self,
        reader: &mut dyn BufRead,
        args_suffix: Option<&str>,
    ) -> PipedEvaluationResult {
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();
        let mut has_error = false;
        let mut had_input = false;

        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {}
                Err(e) => {
                    let _ = writeln!(stderr, "error: {e}");
                    has_error = true;
                    continue;
                }
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            had_input = true;
            let expression = match args_suffix {
                Some(suffix) => format!("{trimmed}{suffix}"),
                None => trimmed.to_string(),
            };

            match self.calculate.execute(&expression) {
                Ok(value) => {
                    let _ = writeln!(stdout, "{value}");
                }
                Err(e) => {
                    let _ = writeln!(stderr, "{}", format_error_with_caret(&expression, &e));
                    has_error = true;
                }
            }
        }

        PipedEvaluationResult {
            has_error,
            had_input,
        }
    }
}
