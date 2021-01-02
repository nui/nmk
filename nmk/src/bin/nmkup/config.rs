#[derive(Debug)]
pub struct Config {
    pub base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let base_url = "https://s3-ap-southeast-1.amazonaws.com/nmk.nuimk.com".to_string();
        Config { base_url }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
