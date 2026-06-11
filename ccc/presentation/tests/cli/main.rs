use assert_cmd::Command;

mod arithmetic;
mod datetime;
mod duration;
mod error_reporting;
mod list_aggregate;
mod list_function;
mod math_function;
mod method_chain;
mod pipe_mode;
mod repl_mode;
mod timestamp;
mod type_cast;

pub fn ccc() -> Command {
    Command::cargo_bin("ccc").unwrap()
}
