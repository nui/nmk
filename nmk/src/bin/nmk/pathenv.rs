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
        let mut f = f.debug_list();
        for p in self.iter() {
            f.entry(p);
        }
        f.finish()
    }
}

impl PathVec {
    pub fn make(&self) -> OsString {
        return env::join_paths(self.clone().unique()).expect("join path error");
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.vec.iter()
    }

    pub fn unique(self) -> Self {
        Self::from_iter(IndexSet::<_>::from_iter(self.vec))
    }

    pub fn no_version_managers(self) -> Self {
        self.vec
            .into_iter()
            .filter(|p| !p.ends_with(".pyenv/shims") && !p.ends_with(".rbenv/shims"))
            .collect()
    }

    pub fn push_front<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_front(path.into())
    }

    #[allow(dead_code)]
    pub fn push_back<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_back(path.into())
    }

    pub fn parse<T: AsRef<OsStr>>(input: T) -> Self {
        let unparsed = input.as_ref();
        if !unparsed.is_empty() {
            env::split_paths(unparsed).filter(|p| p.len() > 0).collect()
        } else {
            Self::new()
        }
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
        let ps = PathVec::parse(OsString::new());
        let actual = ps.make();
        assert_eq!(actual, "");

        // no item add one to front
        let mut ps = PathVec::parse(OsString::new());
        ps.push_front(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // no item add one to back
        let mut ps = PathVec::parse(OsString::new());
        ps.push_back(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // no item add two items to back
        let mut ps = PathVec::parse(OsString::new());
        ps.push_back(FOO.to_string());
        ps.push_back(BAR.to_string());
        let actual = ps.make();
        let expected = OsString::from(format!("{}:{}", FOO, BAR));
        assert_eq!(actual, expected);

        // one item
        let ps = PathVec::parse(OsString::from(FOO));
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // should fix following cases
        let ps = PathVec::parse(OsString::from(":/foo"));
        let actual = ps.make();
        assert_eq!(actual, "/foo");
        let ps = PathVec::parse(OsString::from("/foo:"));
        let actual = ps.make();
        assert_eq!(actual, "/foo");

        // two items
        let input = OsString::from(format!("{}:{}", FOO, BAR));
        let ps = PathVec::parse(&input);
        let actual = ps.make();
        assert_eq!(actual, input);

        // move order correctly
        let input = OsString::from("/a:/b:/c");
        let mut ps = PathVec::parse(&input);
        ps.push_front("/c".to_string());
        let actual = ps.make();
        let expect = OsString::from("/c:/a:/b");
        assert_eq!(actual, expect);

        // remove pyenv, rbenv, and node path
        let input = OsString::from("/a:/home/.pyenv/shims:/home/.rbenv/shims");
        let ps = PathVec::parse(&input).no_version_managers();
        let actual = ps.make();
        assert_eq!(actual, OsString::from("/a"))
    }

    #[test]
    fn test_unique() {
        let input = OsString::from("/a:/b:/a");
        let ps = PathVec::parse(&input);
        let actual = ps.make();
        assert_eq!(actual, OsString::from("/a:/b"))
    }
}
