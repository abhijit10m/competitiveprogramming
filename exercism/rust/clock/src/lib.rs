use chrono::prelude::*;
use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let duration = Duration::minutes(hours as i64 * 60 + minutes as i64 );
        let time = NaiveTime::from_hms_milli(0, 0,0,0)  + duration;
        Clock { hours: time.hour() as i32, minutes: time.minute() as i32}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes != 0 {
            let duration = Duration::minutes(minutes as i64 );
            let time = NaiveTime::from_hms_milli(self.hours as u32, self.minutes as u32, 0, 0) + duration;
            Clock::new( time.hour() as i32, time.minute() as i32)
        } else {
            Clock::new( self.hours, self.minutes)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}


