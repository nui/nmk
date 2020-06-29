use structopt::StructOpt;

use crate::cmdline::Opt;

mod cmdline;
#[macro_use]
mod core;
mod entrypoint;
mod logging;
mod pathenv;
mod terminal;
mod tmux;
mod utils;
mod version;
mod zsh;

fn main() {
    let start = std::time::Instant::now();
    let arg: Opt = Opt::from_args();
    logging::setup(arg.verbosity);
    entrypoint::setup_then_exec(start, arg);
}
