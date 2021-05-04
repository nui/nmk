use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub fn install<P: AsRef<Path>, R: ?Sized>(reader: &mut R, dst: P) -> io::Result<()>
where
    R: Read,
{
    let mut file = open_for_install(dst)?;
    io::copy(reader, &mut file)?;
    file.flush()
}

fn open_for_install<P: AsRef<Path>>(dst: P) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o755)
        .open(dst)
}
