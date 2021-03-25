use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

use bytes::Bytes;
use tar::Archive;

use nmk::gcs::{download_file, ObjectMeta};
use nmk::home::NmkHome;

use crate::build::Target;
use crate::cmdline::CmdOpt;
use crate::os_release::OsReleaseId;

const LIST_OBJECTS_URL: &str =
    "https://storage.googleapis.com/storage/v1/b/nmk.nuimk.com/o?delimiter=/&prefix=nmk-vendor/";
const TAG: &str = "vendor";

pub async fn install(cmd_opt: &CmdOpt, nmk_home: &NmkHome) -> nmk::Result<()> {
    let client = reqwest::Client::new();
    let mut objects: Vec<_> = nmk::gcs::list_objects(&client, LIST_OBJECTS_URL).await?;
    objects.retain(|obj| obj.name.ends_with(".tar.xz"));
    if !cmd_opt.no_filter {
        objects.retain(filter_by_os_release());
        objects.retain(filter_by_arch());
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

fn filter_by_os_release() -> impl FnMut(&ObjectMeta) -> bool {
    use crate::os_release::OsReleaseId::*;
    let pattern = OsReleaseId::parse_os_release().map(|id| match id {
        Amazon => "amazon",
        CentOs => "centos",
        Debian => "debian",
        Ubuntu => "ubuntu",
    });
    move |item: &ObjectMeta| pattern.map_or(true, |pat| item.name.contains(pat))
}

fn filter_by_arch() -> impl FnMut(&ObjectMeta) -> bool {
    let target = Target::detect().expect("Unsupported target");
    const ARM64_TAG: &str = "arm64";
    move |item| {
        let found_tag = item.name.to_lowercase().contains(ARM64_TAG);
        match target {
            Target::Amd64Linux => !found_tag,
            Target::Arm64Linux => found_tag,
            _ => panic!("Unsupported arch"),
        }
    }
}

fn get_display_name(objects: &[ObjectMeta]) -> Vec<&str> {
    objects
        .iter()
        .flat_map(|obj| obj.name.split('/').last())
        .collect()
}

fn remove_dir_contents(path: impl AsRef<Path>) -> io::Result<()> {
    fs::read_dir(path)?.try_for_each(|entry| {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            fs::remove_file(p)
        }
    })
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
        if stdin.read_line(&mut input).is_ok() {
            log::debug!("Input value: {:?}", input);
            if let Ok(choice) = input.trim().parse::<usize>() {
                let index = choice.wrapping_sub(1);
                if let Some(v) = objects.get(index) {
                    return Ok(v);
                }
            }
            println!("Invalid index: {}", input);
        }
        input.clear();
    }
}

fn display_some_os_info() -> io::Result<()> {
    // On CentOS, /etc/os-release doesn't show CentOS minor version
    let infos = ["/etc/centos-release", "/etc/os-release"].iter();
    log::info!("Displaying os information..");
    if let Some(mut f) = infos.map(Path::new).flat_map(File::open).next() {
        io::copy(&mut f, &mut io::stdout())?;
    }
    Ok(())
}

async fn extract_vendor_files<P: AsRef<Path>>(data: Bytes, dst: P) -> io::Result<()> {
    let dst = dst.as_ref();
    let tar_data_stream = xz2::bufread::XzDecoder::new(&*data);
    let mut archive = Archive::new(tar_data_stream);
    log::info!("{}: Installing to {:?}.", TAG, dst);
    archive.unpack(dst)
}
