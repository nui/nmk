use crate::cmdline::Render;
use nmk::tmux::version::Version::*;
use std::io::BufWriter;

pub fn render(opt: &Render) {
    let versions = vec![V26, V27, V28, V29, V29a, V30, V30a, V31, V31a, V31b];
    for v in versions {
        let version_file_name = format!("{}.conf", v.as_str());
        let f = std::fs::File::create(opt.output.join(version_file_name))
            .expect("Unable to create version file");
        let mut f = BufWriter::new(f);
        nmk::tmux::config::render(v, &mut f).expect("Unable to render tmux config");
    }
}
