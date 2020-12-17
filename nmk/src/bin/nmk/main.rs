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
    log::debug!("command line options: {:#?}", opt);
    if let Some(cmd) = opt.cmd {
        use cmdline::SubCommand;
        match cmd {
            SubCommand::Info => commands::info::print_info(),
            SubCommand::Completions(ref c) => commands::completion::gen_completion(c),
        }
    } else {
        entrypoint::main(opt)
    }
}
