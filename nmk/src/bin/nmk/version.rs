use nmk::time::{human_time, seconds_since_build};

pub fn get_verbose_version() -> Option<String> {
    let build_time: i64 = env!("EPOCHSECONDS")
        .parse::<i64>()
        .expect("Unable to get build time");
    let secs_since_build = seconds_since_build(build_time);
    match (secs_since_build, option_env!("GIT_SHORT_SHA")) {
        (Some(secs), Some(sha)) => {
            Some(format!("#{} ({} since last build)", sha, human_time(secs)))
        }
        (Some(secs), None) => Some(format!("({} since last build)", human_time(secs))),
        _ => None,
    }
}
