use std::path::Path;

use os_info::VersionType;

use crate::BoxError;

enum Distro {
    Ubuntu
}

fn find_distro() -> Option<Distro> {
    let info = os_info::get();
    if let VersionType::Custom(code) = info.version().version() {
        if code.eq("18.04") {
            return Some(Distro::Ubuntu);
        }
    }
    None
}

fn find_archive(distro: Distro, username: &str) -> Option<String> {
    let base = "https://storage.googleapis.com/nmk.nuimk.com/nmkpkg/cloudbuild";
    let archive = match distro {
        Distro::Ubuntu => {
            match username {
                n @ "beid" | n @ "nui" | n @ "ubuntu" | n @ "root" => Some(format!("tmux-3.0a--{}--ubuntu-18.04.tar.gz", n)),
                _ => None
            }
        }
    };
    archive.map(|a| format!("{}/{}", base, a))
}

fn find_path(username: &str) -> Option<&'static str> {
    match username {
        "beid" => Some("/home/beid"),
        "nui" => Some("/home/nui"),
        "ubuntu" => Some("/home/ubuntu"),
        "root" => Some("/root"),
        _ => None
    }
}

pub async fn install(nmk_dir: impl AsRef<Path>) -> Result<(), BoxError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experiment() {
        let x = find_distro().and_then(|distro| find_archive(distro, "nui"));
        println!("{:?}", x);
    }
}