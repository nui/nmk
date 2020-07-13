use structopt::StructOpt;

use crate::cmdline::Opt;

mod cmdline;
#[macro_use]
mod core;
mod commands;
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
    match arg.cmd {
        Some(ref sub_command) => {
            use cmdline::SubCommand;
            match *sub_command {
                SubCommand::Info => commands::info::display_info(),
                SubCommand::Completions(ref opt) => commands::completion::completion(opt),
                SubCommand::Other(..) => entrypoint::main(start, arg),
            }
        }
        None => entrypoint::main(start, arg),
    }
}
