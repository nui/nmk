use std::fmt::Formatter;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use dirs::home_dir;

use crate::env::NMK_HOME;

/// Main directory of dotfiles
///
/// - if NMK_HOME is set, use it
/// - if not should default to $HOME/.nmk
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

    pub fn find() -> Option<Self> {
        env::var_os(NMK_HOME)
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|p| p.join(".nmk")))
            .map(From::from)
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
