use std::io::BufWriter;

use nmk::tmux::config::Context;
use nmk::tmux::version::Version::*;

use crate::cmdline::Render;

pub fn render(opt: &Render) -> std::io::Result<()> {
    let versions = vec![V26, V27, V28, V29, V29a, V30, V30a, V31, V31a, V31b];
    for v in versions {
        let version_file_name = format!("{}.conf", v.as_str());
        let f = std::fs::File::create(opt.output.join(version_file_name))?;
        let mut f = BufWriter::new(f);
        let context = Context::default();
        nmk::tmux::config::render(&mut f, &context, v)?;
    }
    Ok(())
}
