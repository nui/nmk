use once_cell::sync::Lazy;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PlatformType {
    Unknown,
    MacOs,
    Linux,
    Arch,
    Alpine,
}

pub fn is_mac() -> bool {
    *PLATFORM == PlatformType::MacOs
}

impl PlatformType {
    pub fn detect() -> PlatformType {
        *PLATFORM
    }
}

static PLATFORM: Lazy<PlatformType> = Lazy::new(what_platform);

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        fn what_platform() -> PlatformType {
            PlatformType::MacOs
        }
    } else if #[cfg(target_os = "linux")] {
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
    } else {
        fn what_platform() -> PlatformType {
            PlatformType::Unknown
        }
    }
}
