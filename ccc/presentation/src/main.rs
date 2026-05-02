mod cli;
mod input_mode;
mod show_builtin;

use std::io;

use clap::Parser;

use infrastructure::evaluator::AstEvaluator;
use infrastructure::parser::PestBasedParser;
use infrastructure::type_checker::AstTypeChecker;
use usecase::calculate_math_expression::CalculateMathExpressionUsecase;
use usecase::evaluate_piped_input::EvaluatePipedInputUsecase;
use usecase::format_error::format_error_with_caret;
use usecase::interactive_repl::InteractiveReplUsecase;

use crate::input_mode::{InputMode, resolve_input_mode};

fn main() {
    let cli = cli::Cli::parse();
    let mode = resolve_input_mode(&cli);

    let parser = PestBasedParser;
    let type_checker = AstTypeChecker;
    let evaluator = AstEvaluator;
    let calculate = CalculateMathExpressionUsecase::new(parser, type_checker, evaluator);

    match mode {
        InputMode::ShowBuiltin => {
            show_builtin::print_builtin_list();
        }
        InputMode::Repl => {
            let repl = InteractiveReplUsecase::new(calculate);
            if let Err(e) = repl.run() {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        InputMode::Expression(expr) => match calculate.execute(&expr) {
            Ok(value) => println!("{value}"),
            Err(e) => {
                eprintln!("{}", format_error_with_caret(&expr, &e));
                std::process::exit(1);
            }
        },
        InputMode::PipeWithArgs(args_expr) => {
            let piped = EvaluatePipedInputUsecase::new(calculate);
            let mut stdin = io::stdin().lock();
            let result = piped.run(&mut stdin, Some(&args_expr));

            if result.has_error {
                std::process::exit(1);
            }
            if result.had_input {
                return;
            }

            // No stdin data: fall back to evaluating args alone
            let parser = PestBasedParser;
            let type_checker = AstTypeChecker;
            let evaluator = AstEvaluator;
            let fallback = CalculateMathExpressionUsecase::new(parser, type_checker, evaluator);
            match fallback.execute(&args_expr) {
                Ok(value) => println!("{value}"),
                Err(e) => {
                    eprintln!("{}", format_error_with_caret(&args_expr, &e));
                    std::process::exit(1);
                }
            }
        }
        InputMode::Pipe => {
            let piped = EvaluatePipedInputUsecase::new(calculate);
            let mut stdin = io::stdin().lock();
            let result = piped.run(&mut stdin, None);
            if result.has_error {
                std::process::exit(1);
            }
        }
        InputMode::NoInput => {
            eprintln!("usage: ccc <expression> or ccc repl");
            std::process::exit(1);
        }
    }
}
