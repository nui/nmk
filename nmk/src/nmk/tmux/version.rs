use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Version {
    V26,
    V27,
    V28,
    V29,
    V29a,
    V30,
    V30a,
    V31,
    V31a,
    V31b,
    V31c,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Version::*;
        let v = match s {
            "2.6" => V26,
            "2.7" => V27,
            "2.8" => V28,
            "2.9" => V29,
            "2.9a" => V29a,
            "3.0" => V30,
            "3.0a" => V30a,
            "3.1" => V31,
            "3.1a" => V31a,
            "3.1b" => V31b,
            "3.1c" => V31c,
            _ => return Err(()),
        };
        Ok(v)
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &'static str {
        use Version::*;
        match *self {
            V26 => "2.6",
            V27 => "2.7",
            V28 => "2.8",
            V29 => "2.9",
            V29a => "2.9a",
            V30 => "3.0",
            V30a => "3.0a",
            V31 => "3.1",
            V31a => "3.1a",
            V31b => "3.1b",
            V31c => "3.1c",
        }
    }
}

#[derive(Debug)]
pub enum ParseVersionError {
    BadVersionOutput(String),
    UnsupportedVersion(String),
}

impl Version {
    // Try parse `tmux -v` result
    pub fn try_from_version_output(version_output: &str) -> Result<Self, ParseVersionError> {
        let version_number = version_output
            .trim()
            .split(" ")
            .nth(1)
            .ok_or_else(|| ParseVersionError::BadVersionOutput(version_output.to_string()))?;
        Self::try_from(version_number)
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryFrom<&str> for Version {
    type Error = ParseVersionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value).map_err(|_| ParseVersionError::UnsupportedVersion(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let tmux_output = "tmux 3.1b";

        let actual = Version::try_from_version_output(tmux_output);
        assert!(matches!(actual, Ok(Version::V31b)));
    }
}
