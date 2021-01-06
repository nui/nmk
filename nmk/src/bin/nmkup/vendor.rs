use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

use bytes::Bytes;
use tar::Archive;

use nmk::gcs::{download_file, ObjectMeta};
use nmk::home::NmkHome;

use crate::cmdline::CmdOpt;
use crate::os_release::OsReleaseId;

const LIST_OBJECTS_URL: &str =
    "https://storage.googleapis.com/storage/v1/b/nmk.nuimk.com/o?delimiter=/&prefix=nmk-vendor/";
const TAG: &str = "vendor";

pub async fn install(cmd_opt: &CmdOpt, nmk_home: &NmkHome) -> nmk::Result<()> {
    let client = reqwest::Client::new();
    let mut objects: Vec<_> = nmk::gcs::list_objects(&client, LIST_OBJECTS_URL)
        .await?
        .into_iter()
        .filter(|obj| obj.name.ends_with(".tar.xz"))
        .collect();
    if !cmd_opt.no_filter {
        objects = filter_by_os_release(objects);
    }
    let obj_meta = select_vendor_files(&objects)?;
    let download_url = obj_meta.media_link.as_str();
    log::info!("{}: Download url {}", TAG, download_url);
    let client = reqwest::Client::new();
    log::debug!("{}: Getting data.", TAG);
    let tar_xz_data = download_file(&client, download_url).await?;
    log::debug!("{}: Received data.", TAG);
    let vendor_dir = nmk_home.nmk_path().vendor();
    if vendor_dir.exists() {
        log::debug!("{}: Removing {:?} content.", TAG, vendor_dir);
        remove_dir_contents(&vendor_dir)?;
    } else {
        fs::create_dir(&vendor_dir)?;
    }
    log::debug!("{}: Extracting data.", TAG);
    extract_vendor_files(tar_xz_data, &vendor_dir).await?;
    log::info!("{}: Done.", TAG);
    Ok(())
}

fn filter_by_os_release(input: Vec<ObjectMeta>) -> Vec<ObjectMeta> {
    use crate::os_release::OsReleaseId::*;
    if let Some(os_release_id) = OsReleaseId::parse_os_release() {
        let filter_key = match os_release_id {
            Amazon => "amazon",
            CentOs => "centos",
            Debian => "debian",
            Ubuntu => "ubuntu",
        };
        input
            .into_iter()
            .filter(|obj| obj.name.contains(filter_key))
            .collect()
    } else {
        input
    }
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
    let stdin = io::stdin();
    let max_index = objects.len();
    assert!(max_index > 0, "Not found any vendor data to select");
    let display_names = get_display_name(objects);
    display_some_os_info()?;
    let mut input = String::new();
    loop {
        println!("Pick vendor files to use?");
        for (index, name) in display_names.iter().enumerate() {
            let numeric_choice = index + 1;
            if max_index < 10 {
                println!(" [{}] {}", numeric_choice, name);
            } else {
                println!(" [{:2}] {}", numeric_choice, name);
            }
        }
        print!("Enter numeric choice:  ");
        io::stdout().flush()?;
        if let Ok(_) = stdin.read_line(&mut input) {
            log::debug!("Input value: {:?}", input);
            if let Ok(index) = input.trim().parse::<usize>() {
                if (1..=max_index).contains(&index) {
                    return Ok(&objects[index - 1]);
                }
            }
            println!("Invalid index: {}", input);
        }
        input.clear();
    }
}

fn display_some_os_info() -> nmk::Result<()> {
    let mut stdout = io::stdout();
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
                io::copy(&mut f, &mut stdout)?;
            }
        }
    }
    Ok(())
}

async fn extract_vendor_files<P: AsRef<Path>>(data: Bytes, dst: P) -> nmk::Result<()> {
    let dst = dst.as_ref();
    let tar_data_stream = xz2::bufread::XzDecoder::new(data.as_ref());
    let mut archive = Archive::new(tar_data_stream);
    log::info!("{}: Installing to {:?}.", TAG, dst);
    archive.unpack(dst)?;
    Ok(())
}
