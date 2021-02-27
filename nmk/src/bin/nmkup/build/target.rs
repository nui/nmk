use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use nmk::arch::detect_current_architecture;

#[derive(Debug)]
pub enum Target {
    Amd64Linux,
    Arm64Linux,
    ArmLinux,
    ArmV7Linux,
    ArmV7LinuxHardFloat,
}

impl Target {
    pub fn detect() -> Result<Self, String> {
        let arch = detect_current_architecture().expect("Unable to detect arch");
        FromStr::from_str(&arch)
    }

    pub fn remote_binary_name(&self, bin: &str) -> String {
        let bin_suffix = match *self {
            Target::Amd64Linux => "x86_64-unknown-linux-musl.xz",
            Target::Arm64Linux => "aarch64-unknown-linux-musl.xz",
            Target::ArmLinux | Target::ArmV7Linux => "arm-unknown-linux-musleabi.xz",
            Target::ArmV7LinuxHardFloat => "armv7-unknown-linux-musleabihf.xz",
        };
        format!("{}-{}", bin, bin_suffix)
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("x86_64-unknown-linux-") => Ok(Target::Amd64Linux),
            x if x.starts_with("aarch64-unknown-linux-") => Ok(Target::Arm64Linux),
            x if ARMV7_HARD_FLOAT.is_match(x) => Ok(Target::ArmV7LinuxHardFloat),
            x if x.starts_with("armv7-unknown-linux") => Ok(Target::ArmV7Linux),
            "arm-unknown-linux-musleabi" => Ok(Target::ArmLinux),
            _ => Err(s.to_string()),
        }
    }
}

static ARMV7_HARD_FLOAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"armv7-unknown-linux.*hf").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armv7_hard_float_matcher() {
        assert!(ARMV7_HARD_FLOAT.is_match("armv7-unknown-linux-gnueabihf"));
        assert!(ARMV7_HARD_FLOAT.is_match("armv7-unknown-linux-musleabihf"));
    }
}
