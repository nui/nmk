use clap::{App, Arg, ArgMatches, Values};

use crate::time::{human_time, seconds_since_build};

#[derive(Debug)]
pub struct Argument<'a> {
    arg: ArgMatches<'a>,
    pub force256color: bool,
    pub force8color: bool,
    pub login: bool,
    pub unicode: bool,
    pub force_unicode: bool,
    pub detach_on_destroy: bool,
    pub autofix: bool,
    pub inception: bool,
    pub debug: bool,
    pub usage: bool,
}

impl<'a> From<ArgMatches<'a>> for Argument<'a> {
    fn from(m: ArgMatches<'a>) -> Self {
        Argument {
            force256color: m.is_present(FORCE_256_COLOR),
            force8color: m.is_present(FORCE_8_COLOR),
            login: m.is_present(LOGIN),
            unicode: m.is_present(UNICODE),
            force_unicode: m.is_present(FORCE_UNICODE),
            detach_on_destroy: m.is_present(DETACH_ON_DESTROY),
            autofix: !m.is_present(NO_AUTOFIX),
            inception: m.is_present(INCEPTION),
            debug: m.is_present(DEBUG),
            usage: m.is_present(USAGE),
            arg: m,
        }
    }
}

impl<'a> Argument<'a> {
    pub fn tmux_args(&self) -> Values {
        self.arg.values_of(TMUX_ARG).unwrap_or_default()
    }

    pub fn socket(&self) -> &str {
        self.arg.value_of(SOCKET).unwrap()
    }
}

const FORCE_256_COLOR: &str = "FORCE_256_COLOR";
const FORCE_8_COLOR: &str = "FORCE_8_COLOR";
const SOCKET: &str = "SOCKET";
const LOGIN: &str = "LOGIN";
const UNICODE: &str = "UNICODE";
const FORCE_UNICODE: &str = "FORCE_UNICODE";
const DETACH_ON_DESTROY: &str = "DETACH_ON_DESTROY";
const NO_AUTOFIX: &str = "NO_AUTOFIX";
const INCEPTION: &str = "INCEPTION";
const DEBUG: &str = "DEBUG";
const TMUX_ARG: &str = "TMUX_ARG";
const USAGE: &str = "USAGE";

fn get_version() -> Option<String> {
    option_env!("SHORT_SHA").map(|short_sha| match seconds_since_build() {
        Some(secs) => format!("#{} ({} since last build)", short_sha, human_time(secs)),
        None => short_sha.to_string(),
    })
}

pub fn parse(unicode: &str) -> Argument {
    let version = get_version().unwrap_or_default();
    const NAME: &str = "nmk.rs";
    App::new(NAME)
        .bin_name(NAME)
        .version(version.as_str())
        .about("An entrypoint for nmk")
        .arg(Arg::with_name(FORCE_256_COLOR)
            .short("2")
            .help("Assume the terminal supports 256 colours")
        )
        .arg(Arg::with_name(FORCE_8_COLOR)
            .short("8")
            .help("Force 8 colours terminal")
        )
        .arg(Arg::with_name(SOCKET)
            .short("L")
            .long("socket")
            .default_value("nmk")
            .value_name("SOCKET")
            .takes_value(true)
            .help("Use a different tmux socket name")
        )
        .arg(Arg::with_name(LOGIN)
            .short("l")
            .long("login")
            .help("Start login shell")
        )
        .arg(Arg::with_name(UNICODE)
            .short("u")
            .long("unicode")
            .help(format!("LANG={}", unicode).as_str())
        )
        .arg(Arg::with_name(FORCE_UNICODE)
            .long("force-unicode")
            .help(format!("LC_ALL={}", unicode).as_str())
        )
        .arg(Arg::with_name(DETACH_ON_DESTROY)
            .long("detach-on-destroy")
            .help("Detach the client when the session is destroyed")
        )
        .arg(Arg::with_name(NO_AUTOFIX)
            .long("no-autofix")
            .help("Disable automatically fix")
        )
        .arg(Arg::with_name(INCEPTION)
            .long("inception")
            .help("Allow nested tmux sessions")
        )
        .arg(Arg::with_name(DEBUG)
            .short("d")
            .long("debug")
            .help("Display debug log")
        )
        .arg(Arg::with_name(USAGE)
            .long("usage")
            .help("Print usage time")
        )
        .arg(Arg::with_name(TMUX_ARG).multiple(true))
        .get_matches()
        .into()
}
