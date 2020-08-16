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
    log::debug!("options: {:#?}", opt);
    match opt.cmd {
        Some(sub_command) => {
            use cmdline::SubCommand;
            match sub_command {
                SubCommand::Info => commands::info::display_info(),
                SubCommand::Completions(ref x) => commands::completion::completion(x),
            }
        }
        None => entrypoint::main(opt),
    }
}
