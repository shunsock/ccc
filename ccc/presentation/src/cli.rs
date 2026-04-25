use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccc", version, about = "CaluCulator Cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Expression to evaluate (e.g. 1 + 2)
    #[arg(trailing_var_arg = true)]
    pub expression: Vec<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start interactive REPL
    Repl,
}
