use crate::ccc;

// --- mean ---

#[test]
fn evaluate_mean_integers() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([1, 2, 3, 4, 5])").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_mean_floats() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([1.0, 2.0, 3.0])").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_mean_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([])").assert();

    // Assert
    result.failure();
}

// --- variance ---

#[test]
fn evaluate_variance_integers() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("variance([1, 2, 3, 4, 5])").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_variance_single_element() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("variance([5])").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_variance_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("variance([])").assert();

    // Assert
    result.failure();
}

// --- max ---

#[test]
fn evaluate_max_integers() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("max([3, 1, 4, 1, 5])").assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_max_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("max([])").assert();

    // Assert
    result.failure();
}

// --- min ---

#[test]
fn evaluate_min_integers() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("min([3, 1, 4, 1, 5])").assert();

    // Assert
    result.success().stdout("1\n");
}

#[test]
fn evaluate_min_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("min([])").assert();

    // Assert
    result.failure();
}

// --- median ---

#[test]
fn evaluate_median_odd() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("median([3, 1, 2])").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_median_even() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("median([1, 2, 3, 4])").assert();

    // Assert
    result.success().stdout("2.5\n");
}

#[test]
fn evaluate_median_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("median([])").assert();

    // Assert
    result.failure();
}

// --- prod ---

#[test]
fn evaluate_prod() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("prod([1,2,3])").assert();

    // Assert
    result.success().stdout("6\n");
}

#[test]
fn evaluate_prod_empty() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("prod([])").assert();

    // Assert
    result.success().stdout("1\n");
}
