use std::fmt::*;
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut correctly_hours = match hours {
            h if h >= 24 => hours % 24,
            h if h < 0 && h >= -24 => h + 24,
            h if h < 0 => h % 24 * -1,
            h => h,
        };
        let correctly_minutes = match minutes {
            m if m >= 60 => {
                let ahead = m / 60;
                if ahead >= 24 {
                    correctly_hours += ahead % 24;
                } else {
                    correctly_hours += ahead;
                }
                m - (60 * ahead)
            }
            m if m < 0 => {
                let ahead = m / 60;
                if ahead >= 24 {
                    correctly_hours += ahead % 24;
                } else {
                    correctly_hours += ahead;
                }
                m - (60 * ahead)
            }
            m => m,
        };

        Clock {
            hours: correctly_hours,
            minutes: correctly_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = (self.minutes + minutes) / 60;
        let new_minutes = (self.minutes + minutes) - (hours * 60);

        Clock {
            hours: self.hours + hours,
            minutes: new_minutes,
        }
    }

    pub fn to_string(&self) -> String {
        let hours = match self.hours {
            h if h < 10 => format!("0{}", self.hours),
            _ => self.hours.to_string(),
        };
        let minutes = match self.minutes {
            m if m < 10 => format!("0{}", self.minutes),
            _ => self.minutes.to_string(),
        };

        String::from(format!("{}:{}", hours, minutes))
    }
}
