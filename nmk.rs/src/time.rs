use std::time::{SystemTime, UNIX_EPOCH};

const DAY_SECS: i64 = 24 * 60 * 60;
const HOUR_SECS: i64 = 60 * 60;
const MINUTE_SECS: i64 = 60;

pub fn human_time(secs: i64) -> String {
    if secs < 0 {
        return format!("{}s", secs);
    }
    let days = secs / 60 / 60 / 24;
    let hours = secs / 60 / 60 % 24;
    let minutes = secs / 60 % 60;
    let seconds = secs % 60;
    let mut result: Vec<String> = Vec::new();
    if secs >= DAY_SECS {
        result.push(format!("{}d", days));
    }
    if secs >= HOUR_SECS {
        result.push(format!("{}h", hours));
    }
    if secs >= MINUTE_SECS {
        result.push(format!("{}m", minutes));
    }
    result.push(format!("{}s", seconds));
    result
        .into_iter()
        .take(2)
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn seconds_since_build() -> Option<i64> {
    let mut result = None;
    if let Some(t) = option_env!("EPOCHSECONDS") {
        if let Ok(build_time) = t.parse::<i64>() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            result = Some(now - build_time);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(human_time(1), "1s");
        assert_eq!(human_time(10), "10s");
        assert_eq!(human_time(59), "59s");
        assert_eq!(human_time(MINUTE_SECS), "1m 0s");
        assert_eq!(human_time(HOUR_SECS), "1h 0m");
        assert_eq!(human_time(HOUR_SECS - 1), "59m 59s");
        assert_eq!(human_time(HOUR_SECS), "1h 0m");
        assert_eq!(human_time(HOUR_SECS + 1), "1h 0m");
        assert_eq!(human_time(DAY_SECS - 1), "23h 59m");
        assert_eq!(human_time(DAY_SECS), "1d 0h");
        assert_eq!(human_time(DAY_SECS + 1), "1d 0h");

        assert_eq!(human_time(-1), "-1s");
        assert_eq!(human_time(-HOUR_SECS), "-3600s");
    }
}
