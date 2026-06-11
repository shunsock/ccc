use crate::ccc;
use predicates::prelude::*;

// --- Error cases ---

#[test]
fn no_arguments_with_empty_stdin_succeeds() {
    // Arrange: assert_cmd provides non-TTY stdin, so empty stdin triggers pipe mode
    let mut cmd = ccc();

    // Act
    let result = cmd.write_stdin("").assert();

    // Assert: empty pipe input produces no output and exits successfully
    result.success().stdout("");
}

#[test]
fn division_by_zero_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1/0").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("division by zero"));
}

#[test]
fn invalid_expression_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("***").assert();

    // Assert
    result.failure();
}

#[test]
fn mixed_type_list_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[1, 2.0, 3]").assert();

    // Assert
    result.failure().stderr(predicate::str::contains(
        "list elements must be the same type",
    ));
}

#[test]
fn unknown_function_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("unknown(1)").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("undefined function"));
}
