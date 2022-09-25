use time::PrimitiveDateTime as DateTime;
use std::{time::Duration, ops::Add};


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_second = Duration::new(1_000_000_000, 0);
    start.add(giga_second)
}
