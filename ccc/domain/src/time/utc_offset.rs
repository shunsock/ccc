/// Timezone offset from UTC in seconds.
///
/// Invariant: `|seconds| < 24h`, the range `chrono::FixedOffset` accepts.
/// Constructing through [`UtcOffset::from_seconds`] guarantees that display
/// formatting can never fail on an out-of-range offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtcOffset(i32);

impl UtcOffset {
    pub const UTC: Self = Self(0);

    const MAX_ABS_SECONDS: i32 = 24 * 3600;

    /// Returns `None` if the offset is outside ±24h (exclusive).
    pub fn from_seconds(seconds: i32) -> Option<Self> {
        (seconds.abs() < Self::MAX_ABS_SECONDS).then_some(Self(seconds))
    }

    pub fn seconds(self) -> i32 {
        self.0
    }

    pub fn is_utc(self) -> bool {
        self.0 == 0
    }
}
