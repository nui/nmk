use crate::cmdline::Opt;

pub fn print_usage_time(opt: &Opt) {
    let before_exec = opt.start_time.elapsed().as_micros();
    if opt.usage {
        println!("{}", before_exec);
    } else {
        log::debug!("usage time: {} μs", before_exec);
    }
}
