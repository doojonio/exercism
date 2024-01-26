use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_IN_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let start = Self {
            hours: 0,
            minutes: 0,
        };
        start.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = (self.hours * 60 + self.minutes + minutes) % MINUTES_IN_DAY;
        if total_minutes < 0 {
            total_minutes = MINUTES_IN_DAY + total_minutes
        }

        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
