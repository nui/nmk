pub mod env_var;
pub mod platform;
pub mod time;

pub fn get_version() -> Option<String> {
    match (time::seconds_since_build(), option_env!("GIT_SHORT_SHA")) {
        (Some(secs), Some(sha)) => Some(format!("#{} ({} since last build)", sha, time::human_time(secs))),
        (Some(secs), None) => Some(format!("({} since last build)", time::human_time(secs))),
        _ => None,
    }
}