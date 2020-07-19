use std::path::PathBuf;
use std::time::Instant;

use once_cell::sync::Lazy;
use structopt::StructOpt;

use crate::version::get_verbose_version;

static VERSION: Lazy<String> = Lazy::new(|| get_verbose_version().unwrap_or_default());

#[structopt(
    name = "nmk",
    about = "Start tmux/zsh with custom configuration from dotfiles",
    version = VERSION.as_str()
)]
#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(
        short = "2",
        help = "Force tmux to assume the terminal supports 256 colours"
    )]
    pub force_256_color: bool,
    #[structopt(
        short = "L",
        long = "socket",
        default_value = "nmk",
        value_name = "NAME",
        help = "Use a different tmux socket name"
    )]
    pub socket: String,
    #[structopt(short = "l", long, help = "Start zsh login shell")]
    pub login: bool,
    #[structopt(long, help = "Detach the client when the session is destroyed")]
    pub detach_on_destroy: bool,
    #[structopt(long, help = "Disable automatically fix")]
    pub no_autofix: bool,
    #[structopt(long, help = "Allow nested tmux sessions")]
    pub inception: bool,
    #[structopt(short, parse(from_occurrences), help = "Request verbose logging")]
    pub verbosity: u8,
    #[structopt(short, help = "Explicitly informs tmux that UTF-8 is supported")]
    pub unicode: bool,
    #[structopt(long, help = "Prints usage time")]
    pub usage: bool,
    #[structopt(long, help = "Display Message of The Day")]
    pub motd: bool,
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
    #[structopt(skip = Instant::now())]
    pub start_time: Instant,
}

impl Opt {
    pub fn args(&self) -> &[String] {
        use SubCommand::*;
        match self.cmd {
            Some(Other(ref args)) => args.as_slice(),
            _ => Default::default(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(about = "Display entrypoint information")]
    Info,
    #[structopt(about = "Generate tab-completion scripts for your shell")]
    Completions(Completion),
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
pub struct Completion {
    #[structopt(short, long, help = "output path, default to standard output")]
    pub output: Option<PathBuf>,
    #[structopt(help = "possible values: zsh, bash, fish, powershell, elvish")]
    pub shell: String,
}
