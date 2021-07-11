use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io};

pub fn install(reader: &mut impl Read, dst: impl AsRef<Path>) -> io::Result<()> {
    let mut file = open_for_install(dst)?;
    io::copy(reader, &mut file)?;
    file.flush()
}

/// Install to tmp location first to avoid text busy
///
/// This is done by install to a temporary file next to `dst` then `fs::rename`
pub fn install_busy(reader: &mut impl Read, dst: impl AsRef<Path>) -> io::Result<()> {
    let dst = dst.as_ref();
    let mut tmp_dst = dst.to_path_buf().into_os_string();
    tmp_dst.push(".temporary-next-version");
    install(reader, &tmp_dst)?;
    fs::rename(&tmp_dst, dst)
}

fn open_for_install<P: AsRef<Path>>(dst: P) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o755)
        .open(dst)
}
