#[cfg(test)]
mod tests {
    use crate::time::{DurationSeconds, EpochSeconds, UtcOffset};
    use crate::value::Value;

    #[test]
    fn display_integer() {
        // Arrange
        let value = Value::Integer(42);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "42");
    }

    #[test]
    fn display_negative_integer() {
        // Arrange
        let value = Value::Integer(-7);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "-7");
    }

    #[test]
    fn display_zero_integer() {
        // Arrange
        let value = Value::Integer(0);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "0");
    }

    #[test]
    fn display_float() {
        // Arrange
        let value = Value::Float(2.5);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "2.5");
    }

    #[test]
    fn display_negative_float() {
        // Arrange
        let value = Value::Float(-2.5);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "-2.5");
    }

    #[test]
    fn display_zero_float() {
        // Arrange
        let value = Value::Float(0.0);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "0");
    }

    #[test]
    fn display_float_whole_number() {
        // Arrange
        let value = Value::Float(5.0);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "5");
    }

    #[test]
    fn display_large_integer() {
        // Arrange
        let value = Value::Integer(i64::MAX);
        let expected = format!("{}", i64::MAX);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn display_min_integer() {
        // Arrange
        let value = Value::Integer(i64::MIN);
        let expected = format!("{}", i64::MIN);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn display_empty_list() {
        // Arrange
        let value = Value::List(vec![]);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "[]");
    }

    #[test]
    fn display_list_with_integers() {
        // Arrange
        let value = Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn display_nested_list() {
        // Arrange
        let value = Value::List(vec![
            Value::List(vec![Value::Integer(1), Value::Integer(2)]),
            Value::List(vec![Value::Integer(3)]),
        ]);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "[[1, 2], [3]]");
    }

    #[test]
    fn display_list_with_mixed_types() {
        // Arrange
        let value = Value::List(vec![Value::Integer(1), Value::Float(2.5)]);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "[1, 2.5]");
    }

    // --- DurationTime display ---

    #[test]
    fn display_duration_time_basic() {
        // Arrange
        let value = Value::DurationTime(DurationSeconds::from_seconds(10 * 3600 + 20 * 60 + 30));

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "10:20:30");
    }

    #[test]
    fn display_duration_time_zero() {
        // Arrange
        let value = Value::DurationTime(DurationSeconds::from_seconds(0));

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "0:00:00");
    }

    #[test]
    fn display_duration_time_negative() {
        // Arrange
        let value = Value::DurationTime(DurationSeconds::from_seconds(-3600));

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "-1:00:00");
    }

    #[test]
    fn display_duration_time_over_24_hours() {
        // Arrange: 26 hours 30 minutes
        let value = Value::DurationTime(DurationSeconds::from_seconds(26 * 3600 + 30 * 60));

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "26:30:00");
    }

    #[test]
    fn display_duration_time_seconds_only() {
        // Arrange
        let value = Value::DurationTime(DurationSeconds::from_seconds(45));

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "0:00:45");
    }

    // --- DateTime display ---

    #[test]
    fn display_datetime_utc() {
        // Arrange: 2026-01-01T00:00:00Z (epoch = 1767225600)
        let value = Value::DateTime {
            epoch: EpochSeconds::from_seconds(1_767_225_600).unwrap(),
            offset: UtcOffset::from_seconds(0).unwrap(),
        };

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "2026-01-01T00:00:00Z");
    }

    #[test]
    fn display_datetime_positive_offset() {
        // Arrange: 2026-01-01T09:00:00+09:00 (same UTC instant as above)
        let value = Value::DateTime {
            epoch: EpochSeconds::from_seconds(1_767_225_600).unwrap(),
            offset: UtcOffset::from_seconds(9 * 3600).unwrap(),
        };

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "2026-01-01T09:00:00+09:00");
    }

    #[test]
    fn display_datetime_negative_offset() {
        // Arrange: 2025-12-31T19:00:00-05:00 (same UTC instant as above)
        let value = Value::DateTime {
            epoch: EpochSeconds::from_seconds(1_767_225_600).unwrap(),
            offset: UtcOffset::from_seconds(-5 * 3600).unwrap(),
        };

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "2025-12-31T19:00:00-05:00");
    }

    #[test]
    fn display_datetime_epoch() {
        // Arrange: Unix epoch
        let value = Value::DateTime {
            epoch: EpochSeconds::from_seconds(0).unwrap(),
            offset: UtcOffset::from_seconds(0).unwrap(),
        };

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "1970-01-01T00:00:00Z");
    }

    // --- Calendar conversion round-trip ---

    #[test]
    fn calendar_round_trip_epoch() {
        // Arrange & Act
        let epoch = EpochSeconds::from_calendar(1970, 1, 1, 0, 0, 0).unwrap();
        let (y, m, d, h, mi, s) = epoch.to_calendar();

        // Assert
        assert_eq!(epoch.seconds(), 0);
        assert_eq!((y, m, d, h, mi, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn calendar_round_trip_2026() {
        // Arrange & Act
        let epoch = EpochSeconds::from_calendar(2026, 6, 15, 12, 30, 45).unwrap();
        let (y, m, d, h, mi, s) = epoch.to_calendar();

        // Assert
        assert_eq!((y, m, d, h, mi, s), (2026, 6, 15, 12, 30, 45));
    }

    #[test]
    fn calendar_leap_year_feb_29() {
        // Arrange & Act
        let epoch = EpochSeconds::from_calendar(2024, 2, 29, 0, 0, 0).unwrap();
        let (y, m, d, h, mi, s) = epoch.to_calendar();

        // Assert
        assert_eq!((y, m, d, h, mi, s), (2024, 2, 29, 0, 0, 0));
    }

    #[test]
    fn calendar_invalid_date_returns_none() {
        // Arrange & Act & Assert
        assert!(EpochSeconds::from_calendar(2025, 2, 29, 0, 0, 0).is_none());
        assert!(EpochSeconds::from_calendar(2026, 13, 1, 0, 0, 0).is_none());
        assert!(EpochSeconds::from_calendar(2026, 1, 1, 25, 0, 0).is_none());
    }

    #[test]
    fn days_in_month_february_leap() {
        use crate::calendar::days_in_month;
        assert_eq!(days_in_month(2024, 2), Some(29));
        assert_eq!(days_in_month(2025, 2), Some(28));
        assert_eq!(days_in_month(2000, 2), Some(29));
        assert_eq!(days_in_month(1900, 2), Some(28));
    }

    // --- Timestamp display ---

    #[test]
    fn display_timestamp_integer() {
        // Arrange
        let value = Value::Timestamp(1234567890.0);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "1234567890");
    }

    #[test]
    fn display_timestamp_float() {
        // Arrange
        let value = Value::Timestamp(1234567890.123);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "1234567890.123");
    }

    #[test]
    fn display_timestamp_zero() {
        // Arrange
        let value = Value::Timestamp(0.0);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "0");
    }

    #[test]
    fn display_timestamp_beyond_i64_keeps_float_value() {
        // Arrange: 1e30 has no fractional part but exceeds i64, so the
        // integer path would saturate it to 9223372036854775807
        let value = Value::Timestamp(1e30);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "1000000000000000000000000000000");
    }

    #[test]
    fn display_timestamp_infinity() {
        // Arrange
        let value = Value::Timestamp(f64::INFINITY);

        // Act
        let result = format!("{value}");

        // Assert
        assert_eq!(result, "inf");
    }
}
