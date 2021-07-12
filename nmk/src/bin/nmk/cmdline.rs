use std::path::PathBuf;
use std::time::Instant;

use once_cell::sync::Lazy;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::version::get_verbose_version;

static VERSION: Lazy<String> = Lazy::new(|| get_verbose_version().expect("missing version info"));

#[derive(Debug, StructOpt)]
#[structopt(name = "nmk", version = VERSION.as_str())]
pub struct CmdOpt {
    #[structopt(
        short = "2",
        help = "Force tmux to assume the terminal supports 256 colours"
    )]
    pub force_256_color: bool,
    #[structopt(
        short = "L",
        long = "socket",
        default_value = "nmk",
        value_name = "name",
        help = "Use a different tmux socket name"
    )]
    pub socket: String,
    #[structopt(
        long,
        value_name = "file",
        help = "Specify an alternative tmux configuration file"
    )]
    pub tmux_conf: Option<PathBuf>,
    #[structopt(short = "l", long, help = "Start zsh login shell")]
    pub login: bool,
    #[structopt(long, help = "Detach the client when the session is destroyed")]
    pub detach_on_destroy: bool,
    #[structopt(short, parse(from_occurrences), help = "Request verbose logging")]
    pub verbosity: u8,
    #[structopt(short, help = "Explicitly informs tmux that UTF-8 is supported")]
    pub unicode: bool,
    #[structopt(long, help = "Prints usage time")]
    pub usage: bool,
    #[structopt(long, help = "Display Message of The Day")]
    pub motd: bool,
    #[structopt(long, help = "Render tmux config then exit")]
    pub render: bool,
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
    #[structopt(skip = Instant::now())]
    pub start_time: Instant,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(about = "Backup files to do clean install")]
    Backup,
    #[structopt(about = "Generate tab-completion scripts for your shell")]
    Completions(Completion),
    #[structopt(about = "Display entrypoint information")]
    Info,
    #[structopt(about = "Setup from local files")]
    Setup(Setup),
    #[structopt(about = "Run tmux command on running tmux server")]
    Tmux(Tmux),
}

#[derive(Debug, StructOpt)]
pub struct Completion {
    #[structopt(short, long, help = "write to file instead of stdout")]
    pub output: Option<PathBuf>,
    #[structopt(possible_values = Shell::variants().as_ref())]
    pub shell: Shell,
}

#[derive(Debug, StructOpt)]
pub struct Setup {
    #[structopt(short, long)]
    pub dotfiles: Option<PathBuf>,
    #[structopt(short, long)]
    pub entrypoint: Option<PathBuf>,
    #[structopt(short, long)]
    pub vendor: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct Tmux {
    #[structopt(value_name = "command", help = "Tmux command")]
    pub args: Vec<String>,
}

pub fn parse() -> CmdOpt {
    CmdOpt::from_args()
}
