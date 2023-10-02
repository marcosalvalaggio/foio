use super::init::{get_home_dir, init_pipeline};
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::Path; // Add this line to bring the Read trait into scope

#[derive(Debug, Deserialize)]
struct Config {
    editor: String,
}

pub fn config_decode() -> String {
    let home_dir = get_home_dir();
    let foio_dir = format!("{}/.foio", home_dir);
    let foio_path = Path::new(&foio_dir);

    if !foio_path.exists() {
        init_pipeline();
    }

    let config_path = foio_path.join("config.json"); // Use Path::join to construct the file path
    let mut file: fs::File = match fs::File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Failed to open the config file.");
        }
    };

    let mut config_content: String = String::new();
    if let Err(_) = file.read_to_string(&mut config_content) {
        panic!("Failed to read the config file.");
    }

    let config: Config = match serde_json::from_str(&config_content) {
        Ok(config) => config,
        Err(_) => {
            panic!("Failed to deserialize the config file.");
        }
    };

    return config.editor;
}
