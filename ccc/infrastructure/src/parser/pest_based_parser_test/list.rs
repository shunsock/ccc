use crate::test_fixture::{add, float, int, list, mul};

use super::parse_expr;

// --- List literals ---

#[test]
fn parse_empty_list() {
    // Arrange
    let input = "[]";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, list(vec![]));
}

#[test]
fn parse_list_single_element() {
    // Arrange
    let input = "[1]";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, list(vec![int(1)]));
}

#[test]
fn parse_list_multiple_elements() {
    // Arrange
    let input = "[1, 2, 3]";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, list(vec![int(1), int(2), int(3)]));
}

#[test]
fn parse_list_with_expressions() {
    // Arrange
    let input = "[1+2, 3*4]";
    let expected = list(vec![add(int(1), int(2)), mul(int(3), int(4))]);

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, expected);
}

#[test]
fn parse_nested_list() {
    // Arrange
    let input = "[[1, 2], [3]]";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(
        result,
        list(vec![list(vec![int(1), int(2)]), list(vec![int(3)])])
    );
}

#[test]
fn parse_list_with_floats() {
    // Arrange
    let input = "[1.5, 2.5]";

    // Act
    let result = parse_expr(input);

    // Assert
    assert_eq!(result, list(vec![float(1.5), float(2.5)]));
}
