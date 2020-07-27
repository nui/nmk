use structopt::StructOpt;

mod cmdline;
mod commands;
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
    let opt = cmdline::Opt::from_args();
    logging::setup(opt.verbosity);
    match opt.cmd {
        Some(ref sub_command) => {
            use cmdline::SubCommand;
            match *sub_command {
                SubCommand::Info => commands::info::display_info(),
                SubCommand::Completions(ref opt) => commands::completion::completion(opt),
                SubCommand::Render(ref opt) => commands::render::render(opt),
                SubCommand::Other(..) => entrypoint::main(opt),
            }
        }
        None => entrypoint::main(opt),
    }
}
