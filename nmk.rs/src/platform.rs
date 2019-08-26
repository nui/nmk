use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum PlatformType {
    Unknown,
    OSX,
    Linux,
    Arch,
    Alpine,
}

pub fn is_alpine() -> bool {
    get() == PlatformType::Alpine
}

pub fn is_arch() -> bool {
    get() == PlatformType::Arch
}

pub fn is_mac() -> bool {
    get() == PlatformType::OSX
}

#[cfg(target_os = "macos")]
fn what_platform() -> PlatformType {
    PlatformType::OSX
}

#[cfg(target_os = "linux")]
fn what_platform() -> PlatformType {
    PlatformType::Linux
}

#[cfg(not(any(target_os = "linux", target_os = "macos",)))]
fn what_platform() -> PlatformType {
    PlatformType::Unknown
}

pub fn get() -> PlatformType {
    let mut platform = what_platform();
    if platform == PlatformType::Linux {
        if PathBuf::from("/etc/alpine-release").exists() {
            platform = PlatformType::Alpine
        } else if PathBuf::from("/etc/arch-release").exists() {
            platform = PlatformType::Arch
        }
    }
    platform
}
