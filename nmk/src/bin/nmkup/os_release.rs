use std::fs;

pub enum OsReleaseId {
    Amazon,
    CentOs,
    Debian,
    Ubuntu,
}

const OS_RELEASE_PATH: &str = "/etc/os-release";

impl OsReleaseId {
    fn from_os_release_str(s: &str) -> Option<Self> {
        let id_line = s.lines().find(|l| l.starts_with("ID="))?;
        let id = id_line.trim_start_matches("ID=").trim_matches('"');
        match id {
            "amzn" => Some(OsReleaseId::Amazon),
            "centos" => Some(OsReleaseId::CentOs),
            "debian" => Some(OsReleaseId::Debian),
            "ubuntu" => Some(OsReleaseId::Ubuntu),
            _ => None,
        }
    }

    pub fn parse_os_release() -> Option<Self> {
        fs::read_to_string(OS_RELEASE_PATH)
            .ok()
            .as_deref()
            .and_then(Self::from_os_release_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_os_release() {
        let actual =
            OsReleaseId::from_os_release_str(include_str!("os-release-data/amazonlinux-2"));
        assert!(matches!(actual, Some(OsReleaseId::Amazon)));

        let actual = OsReleaseId::from_os_release_str(include_str!("os-release-data/centos-7.8"));
        assert!(matches!(actual, Some(OsReleaseId::CentOs)));

        let actual = OsReleaseId::from_os_release_str(include_str!("os-release-data/debian-8"));
        assert!(matches!(actual, Some(OsReleaseId::Debian)));

        let actual = OsReleaseId::from_os_release_str(include_str!("os-release-data/ubuntu-14.04"));
        assert!(matches!(actual, Some(OsReleaseId::Ubuntu)));
    }
}
