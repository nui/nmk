use std::collections::{HashSet, VecDeque};
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

pub struct UniquePath {
    vec: VecDeque<PathBuf>,
}

impl UniquePath {
    pub fn make(&self) -> OsString {
        return env::join_paths(self.unique().iter()).expect("join unique path error");
    }

    pub fn unique(&self) -> Vec<PathBuf> {
        let mut vec = Vec::new();
        let mut set = HashSet::new();
        let valid_path = |p: &&PathBuf| {
            let str_opt = p.to_str();
            str_opt.is_some() && str_opt.unwrap().len() > 0
        };
        for p in self.vec.iter().filter(valid_path) {
            if !set.contains(p) {
                set.insert(p.to_path_buf());
                vec.push(p.to_path_buf());
            }
        }
        vec
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
