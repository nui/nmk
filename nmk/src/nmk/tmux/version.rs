use std::str::FromStr;

use strum::AsStaticRef;
use strum_macros::{AsStaticStr, EnumString};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, AsStaticStr, EnumString)]
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
    pub fn from_version_output(s: &str) -> Result<Self, TmuxVersionError> {
        let version_number = s
            .trim()
            .split_ascii_whitespace()
            .nth(1)
            .ok_or_else(|| TmuxVersionError::BadOutput(s.to_string()))?;
        Self::from_version_number(version_number)
    }

    pub fn from_version_number(s: &str) -> Result<Self, TmuxVersionError> {
        Self::from_str(s).map_err(|_| TmuxVersionError::Unsupported(s.to_string()))
    }

    pub fn as_str(&self) -> &'static str {
        AsStaticRef::as_static(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let tmux_output = "tmux 3.1b";

        let actual = Version::from_version_output(tmux_output);
        assert!(matches!(actual, Ok(Version::V31b)));
    }
}
