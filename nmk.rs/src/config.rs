use std::path::PathBuf;
use std::process::exit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "tmuxSettingEnvs")]
    pub tmux_setting_envs: Vec<String>,
}

#[allow(dead_code)]
pub fn load(nmk_dir: &PathBuf) -> Config {
    let config = nmk_dir.join("config.json");
    let contents = std::fs::read_to_string(&config).unwrap_or_else(|_| {
        error!("cannot open {:?} file", &config);
        exit(1);
    });
    serde_json::from_str(&contents).unwrap_or_else(|_| {
        error!("cannot parse {:?} file", &config);
        exit(1);
    })
}
