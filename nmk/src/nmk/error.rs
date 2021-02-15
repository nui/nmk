use std::fmt::{self, Debug, Display};

pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible
    err: Box<ErrorImpl>,
}

struct ErrorImpl {
    error: Box<dyn std::error::Error>,
    tag: &'static str,
    caller: std::panic::Location<'static>,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = self.err.as_ref();
        write!(
            f,
            "{} at {}:{}:{} {:?}",
            err.tag,
            err.caller.file(),
            err.caller.line(),
            err.caller.column(),
            err.error
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = self.err.as_ref();
        write!(
            f,
            "{} at {}:{}:{} {}",
            err.tag,
            err.caller.file(),
            err.caller.line(),
            err.caller.column(),
            err.error
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
        Self {
            err: Box::new(ErrorImpl { error, tag, caller }),
        }
    }
}

impl_from_error!(reqwest::Error);
impl_from_error!(serde_json::Error);
impl_from_error!(std::io::Error);
impl_from_error!(std::str::Utf8Error);
impl_from_error!(toml::ser::Error);
