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
    #[structopt(long)]
    pub ssh: bool,
    pub tmux_args: Vec<String>,
}
