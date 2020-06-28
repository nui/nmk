use std::path::PathBuf;

use once_cell::sync::Lazy;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum PlatformType {
    Unknown,
    OSX,
    Linux,
    Arch,
    Alpine,
}

pub fn is_alpine() -> bool {
    *PLATFORM == PlatformType::Alpine
}

pub fn is_arch() -> bool {
    *PLATFORM == PlatformType::Arch
}

pub fn is_mac() -> bool {
    *PLATFORM == PlatformType::OSX
}

static PLATFORM: Lazy<PlatformType> = Lazy::new(|| {
    let mut platform = what_platform();
    if platform == PlatformType::Linux {
        if PathBuf::from("/etc/alpine-release").exists() {
            platform = PlatformType::Alpine
        } else if PathBuf::from("/etc/arch-release").exists() {
            platform = PlatformType::Arch
        }
    }
    platform
});

#[cfg(target_os = "macos")]
fn what_platform() -> PlatformType {
    PlatformType::OSX
}

#[cfg(target_os = "linux")]
fn what_platform() -> PlatformType {
    PlatformType::Linux
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn what_platform() -> PlatformType {
    PlatformType::Unknown
}
