use std::io::{self, IsTerminal};

use crate::cli::{Cli, Command, ShowTarget};

/// Determines how the application should process input.
pub enum InputMode {
    /// Interactive REPL session
    Repl,
    /// Show built-in function list
    ShowBuiltin,
    /// Evaluate a single expression from CLI arguments
    Expression(String),
    /// Read expressions from piped stdin, one per line
    Pipe,
    /// Combine each piped stdin line with CLI arguments (e.g. `echo 5 | ccc + 1`)
    PipeWithArgs(String),
    /// No input provided; show usage and exit
    NoInput,
}

pub fn resolve_input_mode(cli: &Cli) -> InputMode {
    match &cli.command {
        Some(Command::Repl) => return InputMode::Repl,
        Some(Command::Show { target }) => match target {
            ShowTarget::Builtin => return InputMode::ShowBuiltin,
        },
        None => {}
    }

    let has_args = !cli.expression.is_empty();
    let is_piped = !io::stdin().is_terminal();
    let args_expr = cli.expression.join("");

    match (has_args, is_piped) {
        (true, true) => InputMode::PipeWithArgs(args_expr),
        (true, false) => InputMode::Expression(args_expr),
        (false, true) => InputMode::Pipe,
        (false, false) => InputMode::NoInput,
    }
}
