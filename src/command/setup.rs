use super::init::{get_home_dir, init_pipeline};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    editor: String,
}

fn config_decode() {
    let home_dir = get_home_dir();
    let foio_dir = format!("{}/.foio", home_dir);
    let foio_path = Path::new(&foio_dir);

    if !foio_path.exists() {
        init_pipeline();
    }

    let config_path = format!("{}/config.json", foio_dir);
    let mut file: fs::File = match fs::File::open(config_path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Failed to open the config file.");
        }
    };

    let mut config_content: String = String::new();
    let config: Config = match serde_json::from_str(&config_content) {
        Ok(config) => config,
        Err(_) => {
            panic!("Failed to deserialize the config file.");
        }
    };
}
