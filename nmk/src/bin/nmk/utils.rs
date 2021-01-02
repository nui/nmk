use crate::cmdline::CmdOpt;

pub fn print_usage_time(cmd_opt: &CmdOpt) {
    let before_exec = cmd_opt.start_time.elapsed().as_micros();
    if cmd_opt.usage {
        println!("{}", before_exec);
    } else {
        log::debug!("usage time: {} μs", before_exec);
    }
}
