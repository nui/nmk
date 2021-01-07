use std::collections::VecDeque;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Debug, Formatter};
use std::iter::FromIterator;
use std::path::PathBuf;

use indexmap::IndexSet;
use nix::NixPath;

#[derive(Clone)]
pub struct PathVec {
    vec: VecDeque<PathBuf>,
}

impl Debug for PathVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.vec, f)
    }
}

impl Default for PathVec {
    fn default() -> Self {
        PathVec::new()
    }
}

impl From<OsString> for PathVec {
    fn from(v: OsString) -> Self {
        Self::from_os_str(&v)
    }
}

impl From<&OsStr> for PathVec {
    fn from(v: &OsStr) -> Self {
        Self::from_os_str(v)
    }
}

impl PathVec {
    pub fn join(&self) -> OsString {
        return env::join_paths(self.clone().unique()).expect("join path error");
    }

    pub fn unique(self) -> Self {
        Self::from_iter(IndexSet::<_>::from_iter(self.vec))
    }

    pub fn without_version_managers(self) -> Self {
        self.vec
            .into_iter()
            .filter(|p| !p.ends_with(".pyenv/shims") && !p.ends_with(".rbenv/shims"))
            .collect()
    }

    pub fn prepend<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_front(path.into())
    }

    #[allow(dead_code)]
    pub fn append<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_back(path.into())
    }

    fn from_os_str(input: &OsStr) -> Self {
        env::split_paths(input).filter(|p| p.len() > 0).collect()
    }

    pub fn new() -> Self {
        Self {
            vec: VecDeque::new(),
        }
    }
}

impl FromIterator<PathBuf> for PathVec {
    fn from_iter<T: IntoIterator<Item = PathBuf>>(iter: T) -> Self {
        Self {
            vec: FromIterator::from_iter(iter),
        }
    }
}

impl IntoIterator for PathVec {
    type Item = PathBuf;
    type IntoIter = std::collections::vec_deque::IntoIter<PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const FOO: &str = "/foo";
    const BAR: &str = "/bar";

    #[test]
    fn test() {
        // no item
        let ps = PathVec::from(OsString::new());
        let actual = ps.join();
        assert_eq!(actual, "");

        // no item add one to front
        let mut ps = PathVec::from(OsString::new());
        ps.prepend(FOO.to_string());
        let actual = ps.join();
        assert_eq!(actual, FOO);

        // no item add one to back
        let mut ps = PathVec::from(OsString::new());
        ps.append(FOO.to_string());
        let actual = ps.join();
        assert_eq!(actual, FOO);

        // no item add two items to back
        let mut ps = PathVec::from(OsString::new());
        ps.append(FOO.to_string());
        ps.append(BAR.to_string());
        let actual = ps.join();
        let expected = OsString::from(format!("{}:{}", FOO, BAR));
        assert_eq!(actual, expected);

        // one item
        let ps = PathVec::from(OsString::from(FOO));
        let actual = ps.join();
        assert_eq!(actual, FOO);

        // should fix following cases
        let ps = PathVec::from(OsString::from(":/foo"));
        let actual = ps.join();
        assert_eq!(actual, "/foo");
        let ps = PathVec::from(OsString::from("/foo:"));
        let actual = ps.join();
        assert_eq!(actual, "/foo");

        // two items
        let input = OsString::from(format!("{}:{}", FOO, BAR));
        let ps = PathVec::from(input.as_os_str());
        let actual = ps.join();
        assert_eq!(actual, input);

        // move order correctly
        let input = OsString::from("/a:/b:/c");
        let mut ps = PathVec::from(input);
        ps.prepend("/c".to_string());
        let actual = ps.join();
        let expect = OsString::from("/c:/a:/b");
        assert_eq!(actual, expect);

        // remove pyenv, rbenv, and node path
        let input = OsString::from("/a:/home/.pyenv/shims:/home/.rbenv/shims");
        let ps = PathVec::from(input).without_version_managers();
        let actual = ps.join();
        assert_eq!(actual, OsString::from("/a"))
    }

    #[test]
    fn test_unique() {
        let input = OsString::from("/a:/b:/a");
        let ps = PathVec::from(input);
        let actual = ps.join();
        assert_eq!(actual, OsString::from("/a:/b"))
    }
}
