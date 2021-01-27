use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use dirs::home_dir;

use crate::env_name::NMK_HOME;

/// Home directory of dotfiles
#[derive(Clone)]
pub struct NmkHome(PathBuf);

impl fmt::Debug for NmkHome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

fn find_path_from_homedir() -> Option<PathBuf> {
    home_dir().map(|p| p.join(".nmk"))
}

fn find_path_from_env() -> Option<PathBuf> {
    env::var_os(NMK_HOME)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

impl NmkHome {
    pub fn is_git(&self) -> bool {
        self.0.join(".git").exists()
    }

    /// Attempt to locate correct NMK_HOME candidate
    /// - if NMK_HOME is set, canonicalize it
    /// - otherwise default to $HOME/.nmk
    ///
    /// Canonicalization is necessary because we use this value in vendored zsh which required
    /// absolute path.
    /// If path doesn't exist, return None
    pub fn locate() -> Option<Self> {
        find_path_from_env()
            .and_then(|p| p.canonicalize().ok())
            .or_else(find_path_from_homedir)
            .filter(|p| p.exists())
            .map(Self::from)
    }

    pub fn find_for_install() -> Option<Self> {
        find_path_from_env()
            .or_else(find_path_from_homedir)
            .map(Self::from)
    }

    pub fn nmk_path(&self) -> NmkPath {
        NmkPath(self.0.as_path())
    }
}

impl From<PathBuf> for NmkHome {
    fn from(inner: PathBuf) -> Self {
        Self(inner)
    }
}

impl AsRef<Path> for NmkHome {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for NmkHome {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl Deref for NmkHome {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct NmkPath<'a>(&'a Path);

impl<'a> NmkPath<'a> {
    pub fn bin(&self) -> PathBuf {
        self.0.join("bin")
    }

    pub fn vendor(&self) -> PathBuf {
        self.0.join("vendor")
    }

    pub fn vendor_bin(&self) -> PathBuf {
        self.vendor().join("bin")
    }

    pub fn vendor_lib(&self) -> PathBuf {
        self.vendor().join("lib")
    }

    pub fn zsh(&self) -> PathBuf {
        self.0.join("zsh")
    }

    pub fn vim(&self) -> PathBuf {
        self.0.join("vim")
    }
}
