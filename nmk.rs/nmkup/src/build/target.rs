use std::str::FromStr;

#[derive(Debug)]
pub enum Target {
    Amd64Linux,
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
            "x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-musl" => Ok(Target::Amd64Linux),
            "armv7-unknown-linux-gnueabihf" => Ok(Target::ArmV7Linux),
            _ => Err(s.to_string())
        }
    }
}