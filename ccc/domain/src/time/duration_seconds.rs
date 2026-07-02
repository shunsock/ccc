/// Signed time span as total seconds.
///
/// Distinguishes spans from epoch positions ([`super::EpochSeconds`]) at the
/// type level, so the two kinds of "seconds" cannot be mixed up.
/// Any i64 is a valid span; arithmetic that would leave the i64 range
/// returns `None` instead of wrapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurationSeconds(i64);

impl DurationSeconds {
    pub fn from_seconds(seconds: i64) -> Self {
        Self(seconds)
    }

    /// Total seconds of `HH:MM:SS` components.
    /// `minutes`/`seconds` are `u8` because literals are validated to 0-59 at parse time.
    /// `None` if the total leaves the i64 range.
    pub fn from_hms(hours: i64, minutes: u8, seconds: u8) -> Option<Self> {
        // The minute/second contribution is < 3600, so only the hour term can overflow.
        hours
            .checked_mul(3600)?
            .checked_add(minutes as i64 * 60 + seconds as i64)
            .map(Self)
    }

    /// Total seconds of day/hour/minute/second components, each an arbitrary i64.
    /// `None` if any intermediate term or the total leaves the i64 range.
    pub fn from_components(days: i64, hours: i64, minutes: i64, seconds: i64) -> Option<Self> {
        days.checked_mul(86400)?
            .checked_add(hours.checked_mul(3600)?)?
            .checked_add(minutes.checked_mul(60)?)?
            .checked_add(seconds)
            .map(Self)
    }

    pub fn seconds(self) -> i64 {
        self.0
    }

    /// `None` if the combined span leaves the i64 range.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// `None` if the combined span leaves the i64 range.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    /// `None` if the span is `i64::MIN` seconds, which has no positive counterpart.
    pub fn checked_neg(self) -> Option<Self> {
        self.0.checked_neg().map(Self)
    }

    /// `None` if the scaled span leaves the i64 range.
    pub fn checked_mul(self, scalar: i64) -> Option<Self> {
        self.0.checked_mul(scalar).map(Self)
    }

    /// Truncating division. `None` for a zero divisor or `i64::MIN / -1`.
    pub fn checked_div(self, divisor: i64) -> Option<Self> {
        self.0.checked_div(divisor).map(Self)
    }
}
