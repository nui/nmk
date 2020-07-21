use nmk::time::{human_time, seconds_since_build};

pub fn get_verbose_version() -> Option<String> {
    let secs_since_build = seconds_since_build();
    match (secs_since_build, option_env!("GIT_SHORT_SHA")) {
        (Some(secs), Some(sha)) => {
            Some(format!("#{} ({} since last build)", sha, human_time(secs)))
        }
        (Some(secs), None) => Some(format!("({} since last build)", human_time(secs))),
        _ => None,
    }
}
