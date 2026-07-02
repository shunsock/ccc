use crate::ccc;

// --- Quoted expression ---

#[test]
fn evaluate_simple_addition() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1+2").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_subtraction() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("10-3").assert();

    // Assert
    result.success().stdout("7\n");
}

#[test]
fn evaluate_multiplication() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3*4").assert();

    // Assert
    result.success().stdout("12\n");
}

#[test]
fn evaluate_division_exact() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("10/2").assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_division_with_remainder() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("7/2").assert();

    // Assert
    result.success().stdout("3.5\n");
}

#[test]
fn evaluate_modulo() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("7%3").assert();

    // Assert
    result.success().stdout("1\n");
}

#[test]
fn evaluate_power() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2^10").assert();

    // Assert
    result.success().stdout("1024\n");
}

#[test]
fn evaluate_parentheses() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("(2+3)*4").assert();

    // Assert
    result.success().stdout("20\n");
}

#[test]
fn evaluate_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1.5+2.5").assert();

    // Assert
    result.success().stdout("4\n");
}

#[test]
fn evaluate_negative_unary() {
    // Arrange: the "--" separator remains supported alongside bare hyphen expressions
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "-5+3"]).assert();

    // Assert
    result.success().stdout("-2\n");
}

#[test]
fn evaluate_negative_unary_without_separator() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("-5 + 3").assert();

    // Assert
    result.success().stdout("-2\n");
}

#[test]
fn evaluate_double_negate_without_separator() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("--5").assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_negative_duration_without_separator() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("-1:30:00").assert();

    // Assert
    result.success().stdout("-1:30:00\n");
}

#[test]
fn evaluate_double_negate() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "- -2000"]).assert();

    // Assert
    result.success().stdout("2000\n");
}

#[test]
fn evaluate_triple_negate() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "- - -3"]).assert();

    // Assert
    result.success().stdout("-3\n");
}

#[test]
fn evaluate_double_negate_in_expression() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "5 + - -3"]).assert();

    // Assert
    result.success().stdout("8\n");
}

#[test]
fn evaluate_double_negate_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "- -1.5"]).assert();

    // Assert
    result.success().stdout("1.5\n");
}

// --- Space-separated expression ---

#[test]
fn evaluate_space_separated() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["2", "+", "3"]).assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_space_separated_complex() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["(", "1", "+", "2", ")", "*", "3"]).assert();

    // Assert
    result.success().stdout("9\n");
}

// --- Power right-associativity ---

#[test]
fn evaluate_power_right_associative() {
    // Arrange: 2^3^2 = 2^(3^2) = 2^9 = 512
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2^3^2").assert();

    // Assert
    result.success().stdout("512\n");
}

// --- ** operator ---

#[test]
fn evaluate_double_star_power() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2 ** 3").assert();

    // Assert
    result.success().stdout("8\n");
}

#[test]
fn evaluate_double_star_right_associative() {
    // Arrange: 2**3**2 = 2**(3**2) = 2**9 = 512
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2**3**2").assert();

    // Assert
    result.success().stdout("512\n");
}

#[test]
fn evaluate_multiply_vs_double_star_precedence() {
    // Arrange: 2 * 3 ** 2 = 2 * 9 = 18
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2 * 3 ** 2").assert();

    // Assert
    result.success().stdout("18\n");
}
