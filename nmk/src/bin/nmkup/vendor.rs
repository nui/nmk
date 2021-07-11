use std::fs::File;
use std::io::{self, BufReader, Write};

use log::{debug, info};

use nmk::gcs::{download_file, ObjectMeta};
use nmk::home::NmkHome;
use nmk::vendor::{extract_vendor_files, prepare_vendor_dir};

use crate::build::Target;
use crate::cmdline::CmdOpt;
use crate::os_release::OsReleaseId;

const LIST_OBJECTS_URL: &str =
    "https://storage.googleapis.com/storage/v1/b/nmk.nuimk.com/o?delimiter=/&prefix=nmk-vendor/";
const TAG: &str = "vendor";

pub fn install(cmd_opt: &CmdOpt, nmk_home: &NmkHome) -> nmk::Result<()> {
    let mut objects: Vec<_> = nmk::gcs::list_objects(LIST_OBJECTS_URL)?;
    objects.retain(|obj| obj.name.ends_with(".tar.xz"));
    if !cmd_opt.no_filter {
        objects.retain(filter_by_os_release());
        objects.retain(filter_by_arch());
    }
    let obj_meta = select_vendor_files(&objects)?;
    let download_url = obj_meta.media_link.as_str();
    info!("{}: Download url {}", TAG, download_url);
    debug!("{}: Getting data.", TAG);
    let tar_xz_data = BufReader::new(download_file(download_url)?);
    debug!("{}: Received data.", TAG);
    let vendor_dir = nmk_home.path().vendor();
    prepare_vendor_dir(&vendor_dir)?;
    debug!("{}: Extracting data.", TAG);
    extract_vendor_files(tar_xz_data, &vendor_dir)?;
    info!("{}: Done.", TAG);
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
    move |item: &ObjectMeta| {
        // Try to filter by os-release data, if we can't determine os-release, don't filter at all.
        pattern.map_or(true, |pat| item.name.contains(pat))
    }
}

fn filter_by_arch() -> impl FnMut(&ObjectMeta) -> bool {
    let target = Target::detect().expect("unsupported target");
    const ARM64_TAG: &str = "arm64";
    move |item| {
        let found_tag = item.name.to_lowercase().contains(ARM64_TAG);
        match target {
            Target::Amd64Linux => !found_tag,
            Target::Arm64Linux => found_tag,
            _ => panic!("unsupported arch"),
        }
    }
}

fn get_display_name(objects: &[ObjectMeta]) -> Vec<&str> {
    objects
        .iter()
        .flat_map(|obj| obj.name.split('/').last())
        .collect()
}

fn select_vendor_files(objects: &[ObjectMeta]) -> nmk::Result<&ObjectMeta> {
    let stdin = io::stdin();
    assert!(!objects.is_empty(), "Not found any vendor data to select");
    let display_names = get_display_name(objects);
    display_some_os_info()?;
    let mut input = String::new();
    loop {
        println!("Pick vendor files to use?");
        for (index, name) in display_names.iter().enumerate() {
            let choice = index + 1;
            println!(" {:2}) {}", choice, name);
        }
        print!("Enter numeric choice:  ");
        io::stdout().flush()?;
        if stdin.read_line(&mut input).is_ok() {
            debug!("Input value: {:?}", input);
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

/// Show OS information to help select correct vendor files
///
/// On CentOS, /etc/os-release doesn't show CentOS minor version
fn display_some_os_info() -> io::Result<()> {
    info!("Displaying os information..");
    let infos = ["/etc/centos-release", "/etc/os-release"].iter();
    if let Some(mut f) = infos.flat_map(File::open).next() {
        io::copy(&mut f, &mut io::stdout())?;
    }
    Ok(())
}
