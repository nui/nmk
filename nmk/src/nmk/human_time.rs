use std::fmt::{self, Display};
use std::time::{SystemTime, UNIX_EPOCH};

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = 60 * MINUTE_SECONDS;
const DAY_SECONDS: u64 = 24 * HOUR_SECONDS;

const ALL_PARTS: u8 = u8::MAX;

#[derive(Copy, Clone)]
pub struct HumanTime(u64);

impl HumanTime {
    pub fn new(secs: u64) -> Self {
        HumanTime(secs)
    }

    pub fn days(self) -> Option<u64> {
        if self.0 >= DAY_SECONDS {
            Some(self.0 / DAY_SECONDS)
        } else {
            None
        }
    }

    pub fn hours(self) -> Option<u64> {
        if self.0 >= HOUR_SECONDS {
            Some(self.0 / HOUR_SECONDS % 24)
        } else {
            None
        }
    }

    pub fn minutes(self) -> Option<u64> {
        if self.0 >= MINUTE_SECONDS {
            Some(self.0 / MINUTE_SECONDS % 60)
        } else {
            None
        }
    }

    pub fn secs(self) -> u64 {
        self.0 % MINUTE_SECONDS
    }

    pub fn to_human(self, max_num_parts: u8) -> String {
        self.parts()
            .take(max_num_parts.into())
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn parts(self) -> Components {
        Components::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct Component {
    value: u64,
    unit: Unit,
}

impl Component {
    fn days(value: u64) -> Self {
        Self {
            value,
            unit: Unit::Day,
        }
    }

    fn hours(value: u64) -> Self {
        Self {
            value,
            unit: Unit::Hour,
        }
    }

    fn minutes(value: u64) -> Self {
        Self {
            value,
            unit: Unit::Minute,
        }
    }

    fn seconds(value: u64) -> Self {
        Self {
            value,
            unit: Unit::Second,
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Unit::*;
        let Self { value, unit } = *self;
        match unit {
            Day => write!(f, "{}d", value),
            Hour => write!(f, "{}h", value),
            Minute => write!(f, "{}m", value),
            Second => write!(f, "{}s", value),
        }
    }
}

#[derive(Clone, Copy)]
enum Unit {
    Day,
    Hour,
    Minute,
    Second,
}

impl Unit {
    fn next(self) -> Option<Self> {
        use Unit::*;
        match self {
            Day => Some(Hour),
            Hour => Some(Minute),
            Minute => Some(Second),
            Second => None,
        }
    }
}

pub struct Components {
    time: HumanTime,
    next_unit: Option<Unit>,
}

impl Components {
    fn new(time: HumanTime) -> Self {
        Components {
            time,
            next_unit: Some(Unit::Day),
        }
    }
}

impl Iterator for Components {
    type Item = Component;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let unit = self.next_unit?;
            self.next_unit = unit.next();
            let component = match unit {
                Unit::Day => self.time.days().map(Component::days),
                Unit::Hour => self.time.hours().map(Component::hours),
                Unit::Minute => self.time.minutes().map(Component::minutes),
                Unit::Second => Some(self.time.secs()).map(Component::seconds),
            };
            if component.is_some() {
                return component;
            }
        }
    }
}

impl Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_human(ALL_PARTS))
    }
}

pub fn seconds_since_build() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|x| x.as_secs().checked_sub(get_build_epoch_seconds()))
}

fn get_build_epoch_seconds() -> u64 {
    env!("EPOCHSECONDS")
        .parse()
        .expect("failed to get build time")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(HumanTime(1).to_human(2), "1s");
        assert_eq!(HumanTime(10).to_human(2), "10s");
        assert_eq!(HumanTime(59).to_human(2), "59s");
        assert_eq!(HumanTime(MINUTE_SECONDS).to_human(2), "1m 0s");
        assert_eq!(HumanTime(HOUR_SECONDS).to_human(2), "1h 0m");
        assert_eq!(HumanTime(HOUR_SECONDS - 1).to_human(2), "59m 59s");
        assert_eq!(HumanTime(HOUR_SECONDS).to_human(2), "1h 0m");
        assert_eq!(HumanTime(HOUR_SECONDS + 1).to_human(2), "1h 0m");
        assert_eq!(HumanTime(DAY_SECONDS - 1).to_human(2), "23h 59m");
        assert_eq!(HumanTime(DAY_SECONDS).to_human(2), "1d 0h");
        assert_eq!(HumanTime(DAY_SECONDS + 1).to_human(2), "1d 0h");
    }
}
