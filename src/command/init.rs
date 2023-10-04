use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Define a struct to represent your JSON data
#[derive(Serialize, Deserialize)]
struct Config {
    editor: String,
}

pub fn get_home_dir() -> String {
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            panic!("Failed to get HOME directory");
        }
    };
    return home_dir;
}

fn create_foio_dir(log: bool) {
    let home_dir = get_home_dir();
    println!("{}", home_dir);
    let foio_dir = format!("{}/.foio", home_dir);
    let foio_path = Path::new(&foio_dir);

    if !foio_path.exists() {
        // If it doesn't exist, create the directory
        if let Err(err) = fs::create_dir(&foio_path) {
            panic!("Error creating .foio directory: {:?}", err);
        } else {
            if log {
                println!("Created .foio directory");
            }
        }
    } else {
        if log {
            println!(".foio directory already exists");
        }
    }
}

fn create_pages_dir(log: bool) {
    let home_dir = get_home_dir();
    let pages_dir = format!("{}/.foio/pages/", home_dir);
    let pages_path = Path::new(&pages_dir);
    if !pages_path.exists() {
        if let Err(err) = fs::create_dir(&pages_path) {
            panic!("Error creating pages directory {:?}", err);
        } else {
            if log {
                println!("Created .foio/pages directory");
            }
        }
    } else {
        if log {
            println!(".foio/pages directory alreadt exist")
        }
    }
}

fn create_config_file() {
    let home_dir = get_home_dir();
    let foio_dir = format!("{}/.foio", home_dir);
    let config_path = PathBuf::from(foio_dir).join("config.json");

    if config_path.exists() {
        println!("config.json already exists at: {:?}", config_path);
        return;
    } else {
        let config_data = Config {
            editor: String::from("vim"),
        };

        let serialized = serde_json::to_string(&config_data).expect("Failed to serialize JSON");

        if let Err(err) = fs::write(&config_path, serialized) {
            panic!("Error creating config.json: {:?}", err);
        } else {
            println!("Created config.json at: {:?}", config_path);
        }
    }
}

fn create_page_file() {
    let home_dir = get_home_dir();
    let foio_dir = format!("{}/.foio", home_dir);
    let page_path = PathBuf::from(foio_dir).join("page.md");
    if page_path.exists() {
        // trucate page.md
        if let Err(err) = fs::write(&page_path, "") {
            panic!("Error truncating page.md: {:?}", err);
        } else {
            println!("Truncated page.md at: {:?}", page_path);
        }
    } else {
        // create page.md
        match fs::File::create(&page_path) {
            Ok(_) => {
                println!("created page.md at: {:?}", page_path);
            }
            Err(err) => {
                panic!("Error creating page.md: {:?}", err);
            }
        }
    }
}

pub fn init_pipeline(log: bool) {
    create_foio_dir(log);
    create_pages_dir(log);
    create_config_file();
    create_page_file();
}
