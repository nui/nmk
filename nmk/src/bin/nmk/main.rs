mod cmdline;
mod commands;
mod entrypoint;
mod logging;
mod path_vec;
mod terminal;
mod tmux;
mod utils;
mod version;
mod zsh;

fn main() -> nmk::Result<()> {
    let cmd_opt = cmdline::parse();
    logging::setup(cmd_opt.verbosity);
    log::debug!("Command line options: {:#?}", cmd_opt);
    if let Some(ref cmd) = cmd_opt.cmd {
        use cmdline::SubCommand::*;
        match cmd {
            Backup => commands::backup::backup()?,
            Completions(c) => commands::completion::gen_completion(c),
            Info => commands::info::print_info()?,
        }
    } else {
        entrypoint::main(cmd_opt)?;
    }
    Ok(())
}
