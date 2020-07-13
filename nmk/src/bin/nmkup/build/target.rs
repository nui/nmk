use std::str::FromStr;

#[derive(Debug)]
pub enum Target {
    Amd64Linux,
    Arm64Linux,
    ArmLinux,
    ArmV7Linux,
}

impl Target {
    pub fn try_parse_env() -> Result<Target, <Self as FromStr>::Err> {
        FromStr::from_str(env!("CARGO_TARGET"))
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("x86_64-unknown-linux-") => Ok(Target::Amd64Linux),
            x if x.starts_with("aarch64-unknown-linux-") => Ok(Target::Arm64Linux),
            x if x.starts_with("armv7-unknown-linux") => Ok(Target::ArmV7Linux),
            "arm-unknown-linux-musleabi" => Ok(Target::ArmLinux),
            _ => Err(s.to_string()),
        }
    }
}
