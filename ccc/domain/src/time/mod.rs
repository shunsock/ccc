//! Time-related value objects.
//!
//! Each type encodes one invariant at construction time, so downstream
//! code (evaluation, display) never needs to re-validate:
//!
//! - [`DurationSeconds`]: a signed time span, distinct from epoch positions
//! - [`EpochSeconds`]: a point in time guaranteed representable by chrono
//! - [`UtcOffset`]: a timezone offset guaranteed displayable

mod duration_seconds;
mod epoch_seconds;
mod utc_offset;

pub use duration_seconds::DurationSeconds;
pub use epoch_seconds::EpochSeconds;
pub use utc_offset::UtcOffset;
