use crate::ccc;

// --- Built-in functions ---

#[test]
fn evaluate_sqrt() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sqrt(16)").assert();

    // Assert
    result.success().stdout("4\n");
}

#[test]
fn evaluate_abs_negative() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("abs(-5)").assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_nested_function() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sqrt(abs(-9))").assert();

    // Assert
    result.success().stdout("3\n");
}

// --- Logarithm ---

#[test]
fn evaluate_log_natural() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("log(1)").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_log_with_base() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("log(2, 8)").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_log_base10() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("log(10, 100)").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_ln() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("ln(1)").assert();

    // Assert
    result.success().stdout("0\n");
}
