use std::fmt::{self, Debug, Display};

pub struct Error {
    error: Box<dyn std::error::Error>,
    tag: &'static str,
    caller: std::panic::Location<'static>,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}:{} {:?}",
            self.tag,
            self.caller.file(),
            self.caller.line(),
            self.caller.column(),
            self.error
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}:{} {}",
            self.tag,
            self.caller.file(),
            self.caller.line(),
            self.caller.column(),
            self.error
        )
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn new(
        error: Box<dyn std::error::Error>,
        tag: &'static str,
        caller: std::panic::Location<'static>,
    ) -> Self {
        Error { error, tag, caller }
    }
}

impl_from_error!(log::SetLoggerError);
impl_from_error!(reqwest::Error);
impl_from_error!(serde_json::Error);
impl_from_error!(std::io::Error);
impl_from_error!(std::path::StripPrefixError);
impl_from_error!(std::str::Utf8Error);
impl_from_error!(toml::ser::Error);
