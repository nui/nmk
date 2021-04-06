use once_cell::sync::Lazy;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum PlatformType {
    Unknown,
    MacOs,
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
    *PLATFORM == PlatformType::MacOs
}

static PLATFORM: Lazy<PlatformType> = Lazy::new(what_platform);

#[cfg(target_os = "macos")]
fn what_platform() -> PlatformType {
    PlatformType::MacOs
}

#[cfg(target_os = "linux")]
fn what_platform() -> PlatformType {
    let exists = |s: &str| std::path::Path::new(s).exists();
    if exists("/etc/alpine-release") {
        PlatformType::Alpine
    } else if exists("/etc/arch-release") {
        PlatformType::Arch
    } else {
        PlatformType::Linux
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn what_platform() -> PlatformType {
    PlatformType::Unknown
}
