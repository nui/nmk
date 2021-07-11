use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

use dirs::home_dir;

use crate::consts::env::NMK_HOME;

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
            .map(Self)
    }

    pub fn find_for_install() -> Option<Self> {
        find_path_from_env()
            .or_else(find_path_from_homedir)
            .map(Self)
    }

    pub fn path(&self) -> &NmkPath {
        NmkPath::new(self.0.as_path())
    }
}

#[repr(transparent)]
// NOTE:
// `NmkPath::new` current implementation relies
// on `NmkPath` being layout-compatible with `Path`
pub struct NmkPath {
    inner: Path,
}

impl NmkPath {
    fn new<P: AsRef<Path> + ?Sized>(p: &P) -> &Self {
        // SAFETY: Self is new type struct with same layout and representation as inner Path
        unsafe { &*(p.as_ref() as *const Path as *const NmkPath) }
    }

    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    pub fn bin(&self) -> PathBuf {
        self.inner.join("bin")
    }

    pub fn dotfiles_file_list(&self) -> PathBuf {
        self.inner.join(".installed-files")
    }

    pub fn dotfiles_meta(&self) -> PathBuf {
        self.inner.join(".dotfiles.meta")
    }

    pub fn entrypoint(&self) -> PathBuf {
        self.bin().join("nmk")
    }

    pub fn entrypoint_meta(&self) -> PathBuf {
        self.inner.join(".nmk.meta")
    }

    pub fn vendor(&self) -> PathBuf {
        self.inner.join("vendor")
    }

    pub fn vendor_bin(&self) -> PathBuf {
        self.vendor().join("bin")
    }

    pub fn vendor_lib(&self) -> PathBuf {
        self.vendor().join("lib")
    }

    pub fn vim(&self) -> PathBuf {
        self.inner.join("vim")
    }

    pub fn zsh(&self) -> PathBuf {
        self.inner.join("zsh")
    }
}

impl AsRef<Path> for NmkPath {
    fn as_ref(&self) -> &Path {
        &self.inner
    }
}

impl AsRef<OsStr> for NmkPath {
    fn as_ref(&self) -> &OsStr {
        self.inner.as_os_str()
    }
}

impl Display for NmkHome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0.display(), f)
    }
}
