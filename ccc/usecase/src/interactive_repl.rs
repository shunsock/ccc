use std::io::{self, BufRead, Write};

use domain::interface::evaluator::CccEvaluator;
use domain::interface::parser::CccParser;
use domain::interface::type_checker::CccTypeChecker;

use crate::calculate_math_expression::CalculateMathExpressionUsecase;
use crate::format_error::format_error_with_caret;

/// Usecase: run an interactive REPL session.
pub struct InteractiveReplUsecase<P, T, E> {
    calculate: CalculateMathExpressionUsecase<P, T, E>,
}

impl<P, T, E> InteractiveReplUsecase<P, T, E>
where
    P: CccParser,
    T: CccTypeChecker,
    E: CccEvaluator,
{
    pub fn new(calculate: CalculateMathExpressionUsecase<P, T, E>) -> Self {
        Self { calculate }
    }

    pub fn run(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        let mut stdout = io::stdout();

        loop {
            write!(stdout, "ccc> ")?;
            stdout.flush()?;

            let mut line = String::new();
            let bytes_read = stdin_lock.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed == "exit" || trimmed == "quit" {
                break;
            }

            match self.calculate.execute(trimmed) {
                Ok(value) => {
                    writeln!(stdout, "{value}")?;
                }
                Err(e) => {
                    let mut stderr = io::stderr();
                    writeln!(stderr, "{}", format_error_with_caret(trimmed, &e))?;
                }
            }
        }

        Ok(())
    }
}
