#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsStaticStr,
    strum::Display,
    strum::EnumString,
)]
pub enum Version {
    #[strum(to_string = "2.6")]
    V26,
    #[strum(to_string = "2.7")]
    V27,
    #[strum(to_string = "2.8")]
    V28,
    #[strum(to_string = "2.9")]
    V29,
    #[strum(to_string = "2.9a")]
    V29a,
    #[strum(to_string = "3.0")]
    V30,
    #[strum(to_string = "3.0a")]
    V30a,
    #[strum(to_string = "3.1")]
    V31,
    #[strum(to_string = "3.1a")]
    V31a,
    #[strum(to_string = "3.1b")]
    V31b,
    #[strum(to_string = "3.1c")]
    V31c,
}

#[derive(Debug)]
pub enum TmuxVersionError {
    BadOutput(String),
    Unsupported(String),
}

impl Version {
    // Try parse `tmux -V` result
    pub fn from_version_output(output: &[u8]) -> Result<Self, TmuxVersionError> {
        let output = String::from_utf8_lossy(output);
        match output.trim().split_ascii_whitespace().nth(1) {
            Some(version_number) => Self::from_version(version_number),
            None => Err(TmuxVersionError::BadOutput(output.into_owned())),
        }
    }

    pub fn from_version(s: &str) -> Result<Self, TmuxVersionError> {
        s.parse()
            .map_err(|_| TmuxVersionError::Unsupported(s.to_string()))
    }

    pub fn as_str(&self) -> &'static str {
        strum::AsStaticRef::as_static(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let tmux_output = b"tmux 3.1b";

        let actual = Version::from_version_output(tmux_output);
        assert!(matches!(actual, Ok(Version::V31b)));
    }
}
