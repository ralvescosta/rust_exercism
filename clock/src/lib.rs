use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 1440;

fn max_minutes_per_day(time: i32) -> i32 {
    time % MINUTES_PER_DAY
}

#[derive(PartialEq, Debug)]
pub struct Clock(i32);

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.0 / MINUTES_PER_HOUR;
        let minutes = self.0 - (hours * MINUTES_PER_HOUR);

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time_in_minutes = (hours * MINUTES_PER_HOUR) + minutes;

        if time_in_minutes < 0 {
            return Clock(MINUTES_PER_DAY - max_minutes_per_day(time_in_minutes.abs()));
        }

        Clock(max_minutes_per_day(time_in_minutes))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes < 0 {
            let new_minutes = max_minutes_per_day(minutes.abs());
            let time_in_minutes = max_minutes_per_day(self.0 - new_minutes);
            if time_in_minutes < 0 {
                return Clock(MINUTES_PER_DAY + time_in_minutes);
            }
            return Clock(time_in_minutes);
        }

        Clock(max_minutes_per_day(self.0 + minutes))
    }
}
