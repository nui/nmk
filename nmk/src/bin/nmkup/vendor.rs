use std::io::Write;
use std::path::Path;
use std::{fs, io};

use bytes::{Buf, Bytes};
use tar::Archive;

use nmk::artifact::{download_file, ObjectMeta};
use nmk::home::NmkHome;
use std::fs::File;

const LIST_OBJECTS_URL: &str =
    "https://storage.googleapis.com/storage/v1/b/nmk.nuimk.com/o?delimiter=/&prefix=nmkpkg/";

pub async fn install(nmk_home: &NmkHome) -> nmk::Result<()> {
    let client = reqwest::Client::new();
    let objects = nmk::artifact::list_objects(&client, LIST_OBJECTS_URL).await?;
    let obj_meta = select_vendor_files(&objects)?;
    let download_url = obj_meta.media_link.as_str();
    log::info!("vendor: Download url {}", download_url);
    let client = reqwest::Client::new();
    log::info!("vendor: Getting data.");
    let tar_xz_data = download_file(&client, download_url).await?;
    log::info!("vendor: Received data.");
    let vendor_path = nmk_home.join("local");
    if vendor_path.exists() {
        log::info!("vendor: Removing {:?} content.", vendor_path);
        remove_dir_contents(&vendor_path)?;
    } else {
        fs::create_dir(&vendor_path)?;
    }
    log::info!("vendor: Extracting data.");
    untar_vendor_files(tar_xz_data, &vendor_path).await?;
    log::info!("vendor: Done.");
    Ok(())
}

fn get_display_name(objects: &[ObjectMeta]) -> Vec<&str> {
    objects
        .iter()
        .flat_map(|obj| obj.name.split("/").last())
        .collect()
}

fn remove_dir_contents(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            fs::remove_dir_all(p)?;
        } else {
            fs::remove_file(p)?;
        }
    }
    Ok(())
}

fn select_vendor_files(objects: &[ObjectMeta]) -> nmk::Result<&ObjectMeta> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let max_index = objects.len();
    assert!(max_index > 0, "Not found any vendor data to select");
    let display_names = get_display_name(objects);
    display_some_os_info()?;
    loop {
        input.clear();
        println!("Pick vendor files to use?");
        for (index, name) in display_names.iter().enumerate() {
            println!(" [{:2}] {}", index + 1, name);
        }
        print!("Enter numeric choice:  ");
        std::io::stdout().flush().expect("Flush fail");
        if let Ok(_) = stdin.read_line(&mut input) {
            log::debug!("Input value: {:?}", input);
            if let Ok(index) = input.trim().parse::<usize>() {
                if (1..=max_index).contains(&index) {
                    return Ok(&objects[index - 1]);
                }
            }
            println!("Invalid index: {}", input);
        }
    }
}

fn display_some_os_info() -> nmk::Result<()> {
    let mut stdout = std::io::stdout();
    let infos = [
        "/etc/os-release",
        "/etc/centos-release",
        "/etc/debian_version",
    ];
    log::info!("Displaying some useful info..");
    for s in infos.iter() {
        let p = Path::new(s);
        if p.exists() {
            if let Ok(mut f) = File::open(p) {
                std::io::copy(&mut f, &mut stdout)?;
            }
        }
    }
    Ok(())
}

async fn untar_vendor_files<P: AsRef<Path>>(data: Bytes, dst: P) -> nmk::Result<()> {
    let dst = dst.as_ref();
    let tar_data_stream = xz2::bufread::XzDecoder::new(data.bytes());
    let mut archive = Archive::new(tar_data_stream);
    log::info!("vendor: Installing to {:?}.", dst);
    archive.unpack(dst)?;
    Ok(())
}
