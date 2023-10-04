use super::init::{get_home_dir, init_pipeline};
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf}; // Add this line to bring the Read trait into scope

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

pub fn generate_foio_script() -> String {
    let script = r#"#!/bin/bash
    
if [ $# -eq 0 ]; then
    echo "No arguments provided. Usage: $0 <your_argument>"
    exit 1
fi

fileType="$1"

cd ~
cd .foio
"#;
    let editor = config_decode();
    let page_script = format!(
        r#"
if [ "$fileType" == "page" ]; then
    {} page.md
fi
"#,
        editor
    );

    let calendar_script = format!(
        r#"
if [ "$fileType" == "page" ]; then
    {} calendar.md
fi
"#,
        editor
    );

    let combined_script = script.to_string() + &page_script + &calendar_script;

    return combined_script;
}

pub fn write_foio_script() {
    let home_dir = get_home_dir();
    let foioscript = generate_foio_script();
    let foio_dir = format!("{}/.foio", home_dir);
    let foioscript_path = PathBuf::from(foio_dir).join("foioscript.sh");
    if foioscript_path.exists() {
        if let Err(err) = fs::write(&foioscript_path, "") {
            panic!("Error truncating page.md: {:?}", err);
        } else {
            println!("Truncated foioscript.sh at: {:?}", foioscript_path);
        }
        if let Err(err) = fs::write(&foioscript_path, foioscript) {
            panic!("Error writing the foioscript.sh {:?}", err);
        } else {
            println!("Created the foioscript.sh file");
        }
    } else {
        // create page.md
        match fs::File::create(&foioscript_path) {
            Ok(_) => {
                println!("created page.md at: {:?}", foioscript_path);
                if let Err(err) = fs::write(&foioscript_path, foioscript) {
                    panic!("Error writing the foioscript.sh {:?}", err);
                } else {
                    println!("Created the foioscript.sh file");
                }
            }
            Err(err) => {
                panic!("Error creating page.md: {:?}", err);
            }
        }
    }
}
