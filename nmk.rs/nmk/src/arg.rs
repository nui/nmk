use clap::{App, Arg, ArgMatches, Values};

use common::get_version;
use common::time::{human_time, seconds_since_build};

#[derive(Debug)]
pub struct Argument<'a> {
    arg: ArgMatches<'a>,
    pub force256color: bool,
    pub login: bool,
    pub detach_on_destroy: bool,
    pub autofix: bool,
    pub inception: bool,
    pub debug: bool,
    pub usage: bool,
    pub ssh: bool,
}

impl<'a> From<ArgMatches<'a>> for Argument<'a> {
    fn from(m: ArgMatches<'a>) -> Self {
        Argument {
            force256color: m.is_present(FORCE_256_COLOR),
            login: m.is_present(LOGIN),
            detach_on_destroy: m.is_present(DETACH_ON_DESTROY),
            autofix: !m.is_present(NO_AUTOFIX),
            inception: m.is_present(INCEPTION),
            debug: m.is_present(DEBUG),
            usage: m.is_present(USAGE),
            ssh: m.is_present(SSH),
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
const SOCKET: &str = "SOCKET";
const LOGIN: &str = "LOGIN";
const DETACH_ON_DESTROY: &str = "DETACH_ON_DESTROY";
const NO_AUTOFIX: &str = "NO_AUTOFIX";
const INCEPTION: &str = "INCEPTION";
const DEBUG: &str = "DEBUG";
const TMUX_ARG: &str = "TMUX_ARG";
const USAGE: &str = "USAGE";
const SSH: &str = "SSH";

pub fn parse() -> Argument<'static> {
    let version = get_version().unwrap_or_default();
    App::new("nmk")
        .version(version.as_str())
        .about("An entrypoint for nmk")
        .arg(Arg::with_name(FORCE_256_COLOR)
            .short("2")
            .help("Assume the terminal supports 256 colours")
        )
        .arg(Arg::with_name(SOCKET)
            .short("L")
            .long("socket")
            .default_value("nmk")
            .value_name("NAME")
            .takes_value(true)
            .help("Use a different tmux socket name")
        )
        .arg(Arg::with_name(LOGIN)
            .short("l")
            .long("login")
            .help("Start zsh login shell")
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
        .arg(Arg::with_name(SSH)
            .long("ssh")
        )
        .arg(Arg::with_name(TMUX_ARG).multiple(true))
        .get_matches()
        .into()
}
