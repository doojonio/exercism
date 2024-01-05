use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASECOND: i64 = 1000 * 1000 * 1000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGASECOND)
}
