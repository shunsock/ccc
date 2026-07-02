use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccc", version, about = "CaluCulator Cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Expression to evaluate (e.g. 1 + 2)
    // CONSTRAINT: clap は先頭ハイフンの位置引数を未知のフラグとして拒否するため、
    // 単項マイナスで始まる式 (例: "-5 + 3") には allow_hyphen_values が必要
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub expression: Vec<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start interactive REPL
    Repl,
    /// Show information
    Show {
        #[command(subcommand)]
        target: ShowTarget,
    },
}

#[derive(Subcommand)]
pub enum ShowTarget {
    /// List all built-in functions
    Builtin,
}
