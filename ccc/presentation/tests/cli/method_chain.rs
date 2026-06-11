use crate::ccc;

// --- Method chain ---

#[test]
fn evaluate_method_chain_sum() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[1, 2, 3].sum()").assert();

    // Assert
    result.success().stdout("6\n");
}

#[test]
fn evaluate_method_chain_len() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[1, 2, 3].len()").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_method_chain_head() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[10, 20, 30].head()").assert();

    // Assert
    result.success().stdout("10\n");
}

#[test]
fn evaluate_method_chain_chained() {
    // Arrange: [1, 2, 3].tail().sum() = sum(tail([1, 2, 3])) = sum([2, 3]) = 5
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[1, 2, 3].tail().sum()").assert();

    // Assert
    result.success().stdout("5\n");
}

#[test]
fn evaluate_method_chain_mean() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("[2, 4, 6].mean()").assert();

    // Assert
    result.success().stdout("4\n");
}
