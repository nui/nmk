use std::fmt::{self, Display};
use std::time::{SystemTime, UNIX_EPOCH};

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = 60 * MINUTE_SECONDS;
const DAY_SECONDS: u64 = 24 * HOUR_SECONDS;

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
        self.0 % 60
    }

    pub fn to_human(&self, max_parts: usize) -> String {
        self.to_parts()
            .into_iter()
            .take(max_parts)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_parts(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(4);
        if let Some(d) = self.days() {
            result.push(format!("{}d", d));
        }
        if let Some(h) = self.hours() {
            result.push(format!("{}h", h));
        }
        if let Some(m) = self.minutes() {
            result.push(format!("{}m", m));
        }
        result.push(format!("{}s", self.secs()));
        result
    }
}

impl Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_parts().join(" "))
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
        .expect("Unable to get build time")
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
