use crate::ccc;

// --- DurationTime literal ---

#[test]
fn evaluate_duration_literal() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("10:20:30").assert();

    // Assert
    result.success().stdout("10:20:30\n");
}

#[test]
fn evaluate_duration_zero() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0:00:00").assert();

    // Assert
    result.success().stdout("0:00:00\n");
}

// --- DurationTime constructor ---

#[test]
fn evaluate_duration_constructor_3_args() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("DurationTime(1, 30, 0)").assert();

    // Assert
    result.success().stdout("1:30:00\n");
}

#[test]
fn evaluate_duration_constructor_4_args() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("DurationTime(1, 2, 30, 0)").assert();

    // Assert
    result.success().stdout("26:30:00\n");
}

// --- MM:SS duration ---

#[test]
fn evaluate_duration_mm_ss() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("10:00").assert();

    // Assert
    result.success().stdout("0:10:00\n");
}

#[test]
fn evaluate_duration_mm_ss_with_seconds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1:30").assert();

    // Assert
    result.success().stdout("0:01:30\n");
}

// --- Duration arithmetic ---

#[test]
fn evaluate_duration_add_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1:00:00 + 0:30:00").assert();

    // Assert
    result.success().stdout("1:30:00\n");
}

#[test]
fn evaluate_duration_multiply_integer() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1:00:00 * 3").assert();

    // Assert
    result.success().stdout("3:00:00\n");
}

// --- sum / mean / max / min / median with duration ---

#[test]
fn evaluate_sum_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sum([10:00:00, 8:00:00])").assert();

    // Assert
    result.success().stdout("18:00:00\n");
}

#[test]
fn evaluate_sum_duration_multiple() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sum([1:00:00, 0:30:00, 0:15:00])").assert();

    // Assert
    result.success().stdout("1:45:00\n");
}

#[test]
fn evaluate_sum_duration_divided() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("sum([10:00:00, 8:00:00])/2").assert();

    // Assert
    result.success().stdout("9:00:00\n");
}

#[test]
fn evaluate_mean_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([0:10:00, 0:20:00, 0:30:00])").assert();

    // Assert
    result.success().stdout("0:20:00\n");
}

#[test]
fn evaluate_max_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("max([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:30:00\n");
}

#[test]
fn evaluate_min_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("min([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:10:00\n");
}

#[test]
fn evaluate_median_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("median([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:20:00\n");
}
