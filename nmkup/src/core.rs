use std::env;
use std::ffi::OsStr;

pub fn set_env<K: AsRef<str>, V: AsRef<OsStr>>(key: K, value: V) {
    let key = key.as_ref();
    let value = value.as_ref();
    env::set_var(key, value);
    log::debug!("export {}={:?}", key, value);
}

#[rustfmt::skip]
#[macro_export]
macro_rules! one_hot {
    ($e: expr) => {if $e {"1"} else {"0"}};
}

#[rustfmt::skip]
#[macro_export]
macro_rules! on_off {
    ($e: expr) => {if $e {"on"} else {"off"}};
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_hot() {
        assert_eq!(one_hot!(true), "1");
        assert_eq!(one_hot!(false), "0");
    }

    #[test]
    fn on_off() {
        assert_eq!(on_off!(true), "on");
        assert_eq!(on_off!(false), "off");
    }
}
