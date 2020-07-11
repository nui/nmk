use std::fmt::Formatter;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use dirs::home_dir;

use crate::env_name::NMK_HOME;

/// Main directory of dotfiles
pub struct NmkHome(PathBuf);

impl fmt::Debug for NmkHome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl NmkHome {
    pub fn is_git(&self) -> bool {
        self.0.join(".git").exists()
    }

    fn _find(canonicalize: bool) -> Option<Self> {
        env::var_os(NMK_HOME)
            .map(PathBuf::from)
            .and_then(|p| {
                if canonicalize {
                    p.canonicalize().ok()
                } else {
                    Some(p)
                }
            })
            .or_else(|| home_dir().map(|p| p.join(".nmk")))
            .map(From::from)
    }

    /// Attempt to find correct NMK_HOME candidate
    /// - if NMK_HOME is set, canonicalize it
    /// - otherwise default to $HOME/.nmk
    ///
    /// Absolute path is necessary because we use this value in vendored zsh.
    pub fn find() -> Option<Self> {
        Self::_find(true)
    }

    /// Like find but don't canonicalize (it fail if directory doesn't exist)
    pub fn find_for_install() -> Option<Self> {
        Self::_find(false)
    }
}

impl From<PathBuf> for NmkHome {
    fn from(inner: PathBuf) -> Self {
        Self(inner)
    }
}

impl Deref for NmkHome {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for NmkHome {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
