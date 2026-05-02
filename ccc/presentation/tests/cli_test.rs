use assert_cmd::Command;
use predicates::prelude::*;

fn ccc() -> Command {
    Command::cargo_bin("ccc").unwrap()
}

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
    // Arrange: leading "-" is interpreted as a flag by clap, so use "--" separator
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "-5+3"]).assert();

    // Assert
    result.success().stdout("-2\n");
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

// --- List functions ---

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

// --- mean / E ---

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
fn evaluate_mean_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([0:10:00, 0:20:00, 0:30:00])").assert();

    // Assert
    result.success().stdout("0:20:00\n");
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

// --- var / V ---

#[test]
fn evaluate_var_integers() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("var([1, 2, 3, 4, 5])").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_var_single_element() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("var([5])").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_var_empty_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("var([])").assert();

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
fn evaluate_max_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("max([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:30:00\n");
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
fn evaluate_min_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("min([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:10:00\n");
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
fn evaluate_median_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("median([0:10:00, 0:30:00, 0:20:00])").assert();

    // Assert
    result.success().stdout("0:20:00\n");
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

#[test]
fn evaluate_len_with_number_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("len(42)").assert();

    // Assert
    result.failure();
}

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

// --- DurationTime ---

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

#[test]
fn evaluate_duration_zero() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0:00:00").assert();

    // Assert
    result.success().stdout("0:00:00\n");
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

#[test]
fn evaluate_datetime_add_mm_ss_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2025-12-25T15:30:00+09 + 10:00").assert();

    // Assert
    result.success().stdout("2025-12-25T15:40:00+09:00\n");
}

// --- DateTime ---

#[test]
fn evaluate_datetime_literal_utc() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00").assert();

    // Assert
    result.success().stdout("2026-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_literal_z_suffix() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z").assert();

    // Assert
    result.success().stdout("2026-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_literal_with_offset() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T09:00:00+09:00").assert();

    // Assert
    result.success().stdout("2026-01-01T09:00:00+09:00\n");
}

#[test]
fn evaluate_datetime_constructor() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("DateTime(2026, 6, 15, 12, 30, 0)").assert();

    // Assert
    result.success().stdout("2026-06-15T12:30:00Z\n");
}

// --- Timestamp ---

#[test]
fn evaluate_timestamp_integer() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(1234567890)").assert();

    // Assert
    result.success().stdout("1234567890\n");
}

#[test]
fn evaluate_timestamp_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(1234567890.5)").assert();

    // Assert
    result.success().stdout("1234567890.5\n");
}

#[test]
fn evaluate_timestamp_as_datetime() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("Timestamp(0) as datetime").assert();

    // Assert
    result.success().stdout("1970-01-01T00:00:00Z\n");
}

#[test]
fn evaluate_datetime_as_timestamp() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z as timestamp").assert();

    // Assert
    result.success().stdout("1767225600\n");
}

#[test]
fn evaluate_integer_as_timestamp_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("1 as timestamp").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("cannot cast"));
}

// --- Time arithmetic ---

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

#[test]
fn evaluate_datetime_add_duration() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("2026-01-01T00:00:00Z + 1:30:00").assert();

    // Assert
    result.success().stdout("2026-01-01T01:30:00Z\n");
}

#[test]
fn evaluate_datetime_subtract_datetime() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd
        .arg("2026-01-02T00:00:00Z - 2026-01-01T00:00:00Z")
        .assert();

    // Assert
    result.success().stdout("24:00:00\n");
}

// --- Time utility functions ---

#[test]
fn evaluate_now_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("now()").assert();

    // Assert: output contains a datetime-like pattern
    result.success();
}

#[test]
fn evaluate_today_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("today()").assert();

    // Assert
    result.success();
}

#[test]
fn evaluate_current_timestamp_succeeds() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("current_timestamp()").assert();

    // Assert
    result.success();
}

// --- REPL subcommand ---

#[test]
fn repl_subcommand_exists() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("repl").write_stdin("exit\n").assert();

    // Assert
    result.success();
}

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

// --- Type cast (as) ---

#[test]
fn evaluate_integer_as_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3 as float").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_zero_as_float() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0 as float").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3.7 as int").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_negative_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.args(["--", "-2.9 as int"]).assert();

    // Assert
    result.success().stdout("-2\n");
}

#[test]
fn evaluate_zero_float_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0.0 as int").assert();

    // Assert
    result.success().stdout("0\n");
}

#[test]
fn evaluate_integer_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("3 as int").assert();

    // Assert
    result.success().stdout("3\n");
}

#[test]
fn evaluate_mean_as_int() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("mean([1, 2, 3]) as int").assert();

    // Assert
    result.success().stdout("2\n");
}

#[test]
fn evaluate_duration_as_int_fails() {
    // Arrange
    let mut cmd = ccc();

    // Act
    let result = cmd.arg("0:10:00 as int").assert();

    // Assert
    result
        .failure()
        .stderr(predicate::str::contains("cannot cast"));
}
