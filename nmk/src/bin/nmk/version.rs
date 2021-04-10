use nmk::human_time::{seconds_since_build, HumanTime};

pub fn get_verbose_version() -> Option<String> {
    let secs_since_build = seconds_since_build();
    match (secs_since_build, option_env!("GIT_SHORT_SHA")) {
        (Some(secs), Some(sha)) => Some(format!(
            "#{} ({} since last build)",
            sha,
            HumanTime::new(secs).to_human(2)
        )),
        (Some(secs), None) => Some(format!(
            "({} since last build)",
            HumanTime::new(secs).to_human(2)
        )),
        _ => None,
    }
}
