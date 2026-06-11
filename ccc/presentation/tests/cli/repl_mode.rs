use crate::ccc;
use predicates::prelude::*;

// --- REPL subcommand ---

#[test]
fn repl_subcommand_exists() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("repl").write_stdin("exit\n").assert();

    // Assert
    result.success();
}

// --- Show builtin subcommand ---

#[test]
fn show_builtin_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["show", "builtin"]).assert();

    // Assert
    result
        .success()
        .stdout(predicate::str::contains("Built-in Functions:"));
}

#[test]
fn show_builtin_contains_all_function_names() {
    // Arrange
    let mut cmd = ccc();
    let expected_names = [
        "sqrt",
        "abs",
        "sin",
        "cos",
        "tan",
        "arcsin",
        "arccos",
        "arctan",
        "log",
        "ln",
        "floor",
        "ceil",
        "round",
        "mean",
        "variance",
        "max",
        "min",
        "median",
        "len",
        "sum",
        "prod",
        "head",
        "tail",
        "DurationTime",
        "DateTime",
        "Timestamp",
        "now",
        "today",
        "current_timestamp",
    ];

    // Act
    let output = cmd.args(["show", "builtin"]).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Assert
    for name in expected_names {
        assert!(
            stdout.contains(name),
            "expected output to contain '{name}', but it was missing"
        );
    }
}
