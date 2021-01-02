use structopt::StructOpt;

mod cmdline;
mod commands;
mod core;
mod entrypoint;
mod logging;
mod path_vec;
mod terminal;
mod tmux;
mod utils;
mod version;
mod zsh;

fn main() {
    let cmd_opt = cmdline::CmdOpt::from_args();
    logging::setup(cmd_opt.verbosity);
    log::debug!("command line options: {:#?}", cmd_opt);
    if let Some(cmd) = cmd_opt.cmd {
        use cmdline::SubCommand;
        match cmd {
            SubCommand::Info => commands::info::print_info(),
            SubCommand::Completions(ref c) => commands::completion::gen_completion(c),
        }
    } else {
        entrypoint::main(cmd_opt)
    }
}
