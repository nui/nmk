use std::collections::VecDeque;
use std::env;
use std::ffi::{OsStr, OsString};
use std::iter::FromIterator;
use std::path::PathBuf;

use indexmap::IndexSet;

pub struct PathVec {
    vec: VecDeque<PathBuf>,
}

impl PathVec {
    pub fn make(&self) -> OsString {
        return env::join_paths(self.unique()).expect("join path error");
    }

    pub fn iter(&self) -> impl Iterator<Item=&PathBuf> {
        self.vec.iter()
    }

    pub fn unique(&self) -> Self {
        self.vec.clone().into_iter()
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn no_version_managers(&self) -> Self {
        let vec = self.vec.clone().into_iter().filter(|x| {
            !x.ends_with(".pyenv/shims") && !x.ends_with(".rbenv/shims")
        }).collect();
        Self {
            vec
        }
    }

    pub fn push_front<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_front(path.into())
    }

    #[allow(dead_code)]
    pub fn push_back<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_back(path.into())
    }

    pub fn parse<T: AsRef<OsStr>>(input: T) -> Self {
        let mut vec = VecDeque::new();
        let unparsed = input.as_ref();
        if !unparsed.is_empty() {
            for p in env::split_paths(unparsed) {
                vec.push_back(p);
            }
        }
        Self { vec }
    }

    pub fn new() -> Self {
        Self {
            vec: VecDeque::new(),
        }
    }
}

impl FromIterator<PathBuf> for PathVec {
    fn from_iter<T: IntoIterator<Item=PathBuf>>(iter: T) -> Self {
        Self {
            vec: iter.into_iter().collect()
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
        let ps = PathVec::parse(OsString::from(""));
        let actual = ps.make();
        assert_eq!(actual, "");

        // no item add one to front
        let mut ps = PathVec::parse(OsString::from(""));
        ps.push_front(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // no item add one to back
        let mut ps = PathVec::parse(OsString::from(""));
        ps.push_back(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // one item
        let ps = PathVec::parse(OsString::from(FOO));
        let actual = ps.make();
        assert_eq!(actual, FOO);

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
}
