use std::env;
use std::ffi::OsStr;

pub fn set_env<K: AsRef<str>, V: AsRef<OsStr>>(key: K, value: V) {
    let key = key.as_ref();
    let value = value.as_ref();
    env::set_var(key, value);
    log::debug!("export {}={:?}", key, value);
}
