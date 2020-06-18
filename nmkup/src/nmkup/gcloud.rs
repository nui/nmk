use std::str::FromStr;

use serde_json::Value;
use std::convert::TryFrom;

pub struct MetaData {
    inner: Value,
}

impl MetaData {
    pub fn md5(&self) -> Option<&str> {
        self.inner.get("md5Hash").and_then(Value::as_str)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self.inner).expect("fail serialize value")
    }
}

impl FromStr for MetaData {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map(|v| Self { inner: v })
    }
}

impl TryFrom<&[u8]> for MetaData {
    type Error = serde_json::error::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let text = String::from_utf8_lossy(value);
        serde_json::from_str(&text).map(|v| Self { inner: v })
    }
}
