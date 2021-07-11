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
    let mut cmd_opt = cmdline::parse();
    logging::setup(cmd_opt.verbosity);
    log::debug!("Command line options: {:#?}", cmd_opt);
    if let Some(cmd) = cmd_opt.cmd.take() {
        use cmdline::SubCommand::*;
        match cmd {
            Backup => commands::backup::backup()?,
            Completions(c) => commands::completion::generate_completion(c),
            Info => commands::info::print_info()?,
            Setup(v) => commands::setup::setup(v)?,
        }
    } else {
        entrypoint::main(cmd_opt)?;
    }
    Ok(())
}
