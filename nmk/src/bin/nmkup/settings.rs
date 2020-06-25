use crate::cmdline::Opt;

#[derive(Debug)]
pub struct Settings {
    pub base_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        let base_url = "https://s3-ap-southeast-1.amazonaws.com/nmk.nuimk.com".to_string();
        Settings { base_url }
    }
}

impl Settings {
    pub fn new(_opt: &Opt) -> Self {
        let s = Settings::default();
        s
    }
}
