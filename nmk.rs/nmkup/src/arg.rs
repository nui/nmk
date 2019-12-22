use clap::{App, Arg, ArgMatches};

#[derive(Debug)]
pub struct Argument<'a> {
    arg: ArgMatches<'a>,
    pub debug: bool,
    pub force: bool,
}

impl<'a> From<ArgMatches<'a>> for Argument<'a> {
    fn from(m: ArgMatches<'a>) -> Self {
        Argument {
            debug: m.is_present(DEBUG),
            force: m.is_present(FORCE),
            arg: m,
        }
    }
}

const DEBUG: &str = "DEBUG";
const FORCE: &str = "FORCE";

pub fn parse() -> Argument<'static> {
    App::new("nmkup")
        .about("All in one binary to setup nmk")
        .arg(Arg::with_name(DEBUG)
            .short("d")
            .long("debug")
            .help("Display debug log")
        )
        .arg(Arg::with_name(FORCE)
            .short("f")
            .long("force")
            .help("Force install"))
        .get_matches()
        .into()
}
