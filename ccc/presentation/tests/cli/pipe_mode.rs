use crate::ccc;

// --- Pipe input (stdin) ---

#[test]
fn pipe_single_line() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.write_stdin("1+2\n").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn pipe_single_line_without_trailing_newline() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.write_stdin("5*3").assert();

    // Assert
    result.success().stdout("15\n");
}

// --- Pipe input with arguments (stdin + args combined) ---

#[test]
fn pipe_with_args_addition() {
    // Arrange: echo 5 | ccc + 1 → "5+1" → 6
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["+", "1"]).write_stdin("5\n").assert();

    // Assert
    result.success().stdout("6\n");
}

#[test]
fn pipe_with_args_multiplication() {
    // Arrange: echo 3 | ccc * 2 + 1 → "3*2+1" → 7
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["*", "2", "+", "1"]).write_stdin("3\n").assert();

    // Assert
    result.success().stdout("7\n");
}

#[test]
fn pipe_with_args_multiline() {
    // Arrange: printf "5\n10" | ccc + 1 → "6\n11"
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["+", "1"]).write_stdin("5\n10\n").assert();

    // Assert
    result.success().stdout("6\n11\n");
}
