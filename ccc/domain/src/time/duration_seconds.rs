use std::ops::{Add, Div, Mul, Neg, Sub};

/// Signed time span as total seconds.
///
/// Distinguishes spans from epoch positions ([`super::EpochSeconds`]) at the
/// type level, so the two kinds of "seconds" cannot be mixed up.
/// Any i64 is a valid span; arithmetic follows plain integer semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurationSeconds(i64);

impl DurationSeconds {
    pub fn from_seconds(seconds: i64) -> Self {
        Self(seconds)
    }

    /// Total seconds of `HH:MM:SS` components.
    /// `minutes`/`seconds` are `u8` because literals are validated to 0-59 at parse time.
    pub fn from_hms(hours: i64, minutes: u8, seconds: u8) -> Self {
        Self(hours * 3600 + minutes as i64 * 60 + seconds as i64)
    }

    pub fn seconds(self) -> i64 {
        self.0
    }
}

impl Add for DurationSeconds {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for DurationSeconds {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Neg for DurationSeconds {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Mul<i64> for DurationSeconds {
    type Output = Self;
    fn mul(self, scalar: i64) -> Self {
        Self(self.0 * scalar)
    }
}

/// Caller must reject a zero divisor first; division itself truncates.
impl Div<i64> for DurationSeconds {
    type Output = Self;
    fn div(self, divisor: i64) -> Self {
        Self(self.0 / divisor)
    }
}
