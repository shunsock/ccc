use crate::ccc;

// --- len ---

#[test]
fn evaluate_len() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("len([1,2,3])").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_len_empty() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("len([])").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_len_with_number_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("len(42)").assert();

    // Assert
    result.failure();
}

// --- sum ---

#[test]
fn evaluate_sum() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sum([1,2,3])").assert();

    // Assert
    result.success().stdout("6\n");
}

#[test]
fn evaluate_sum_empty() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sum([])").assert();

    // Assert
    result.success().stdout("0\n");
}

// --- head / tail ---

#[test]
fn evaluate_head() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("head([1,2,3])").assert();

    // Assert
    result.success().stdout("1\n");
}

#[test]
fn evaluate_tail() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("tail([1,2,3])").assert();

    // Assert
    result.success().stdout("[2, 3]\n");
}

#[test]
fn evaluate_tail_single_element() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("tail([1])").assert();

    // Assert
    result.success().stdout("[]\n");
}

#[test]
fn evaluate_head_empty_list_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("head([])").assert();

    // Assert
    result.failure();
}

#[test]
fn evaluate_tail_empty_list_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("tail([])").assert();

    // Assert
    result.failure();
}
