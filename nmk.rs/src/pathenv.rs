use std::collections::{HashSet, VecDeque};
use std::env;
use std::ffi::{OsStr, OsString};
use std::iter::Filter;
use std::path::PathBuf;

use nix::NixPath;

pub struct UniquePath {
    vec: VecDeque<PathBuf>,
}

pub struct Iter<'a, T> {
    filter: Filter<T, fn(&&PathBuf) -> bool>,
    set: HashSet<&'a PathBuf>,
}

impl<'a, T> From<T> for Iter<'a, T> where
    T: Iterator<Item=&'a PathBuf> {
    fn from(iter: T) -> Self {
        Self {
            filter: iter.filter(|p: &&PathBuf| p.len() > 0),
            set: HashSet::new(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: Iterator<Item=&'a PathBuf> {
    type Item = &'a PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let set = &self.set;
        self.filter
            .find(|&x| !set.contains(x))
            .map(|x| {
                self.set.insert(x);
                x
            })
    }
}

impl UniquePath {
    pub fn make(&self) -> OsString {
        return env::join_paths(self.unique()).expect("join unique path error");
    }

    pub fn unique(&self) -> impl Iterator<Item=&PathBuf> {
        Iter::from(self.vec.iter())
    }

    pub fn push_front<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_front(path.into())
    }

    pub fn push_back<T: Into<PathBuf>>(&mut self, path: T) {
        self.vec.push_back(path.into())
    }

    pub fn parse<T: AsRef<OsStr>>(input: T) -> Self {
        let mut vec = VecDeque::new();
        for s in env::split_paths(input.as_ref()) {
            vec.push_back(s);
        }
        Self { vec }
    }

    pub fn new() -> Self {
        Self {
            vec: VecDeque::new(),
        }
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
        let ps = UniquePath::parse(OsString::from(""));
        let actual = ps.make();
        assert_eq!(actual, "");

        // no item add one to front
        let mut ps = UniquePath::parse(OsString::from(""));
        ps.push_front(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // no item add one to back
        let mut ps = UniquePath::parse(OsString::from(""));
        ps.push_back(FOO.to_string());
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // one item
        let ps = UniquePath::parse(OsString::from(FOO));
        let actual = ps.make();
        assert_eq!(actual, FOO);

        // two items
        let input = OsString::from(format!("{}:{}", FOO, BAR));
        let ps = UniquePath::parse(&input);
        let actual = ps.make();
        assert_eq!(actual, input);

        // move order correctly
        let input = OsString::from("/a:/b:/c");
        let mut ps = UniquePath::parse(&input);
        ps.push_front("/c".to_string());
        let actual = ps.make();
        let expect = OsString::from("/c:/a:/b");
        assert_eq!(actual, expect);
    }
}
