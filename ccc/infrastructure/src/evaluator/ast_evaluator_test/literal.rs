use crate::test_fixture::{float, int};

use super::eval;

// --- Literals ---

#[test]
fn eval_integer() {
    // Arrange
    let expression = int(42);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), domain::value::Value::Integer(42));
}

#[test]
fn eval_float() {
    // Arrange
    let expression = float(2.5);

    // Act
    let result = eval(expression);

    // Assert
    assert_eq!(result.unwrap(), domain::value::Value::Float(2.5));
}
