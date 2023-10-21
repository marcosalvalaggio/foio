use super::init::{get_home_dir, init_pipeline};
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf}; // Add this line to bring the Read trait into scope
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Config {
    editor: String,
}

pub fn config_decode(log: bool) -> String {
    let home_dir = get_home_dir();
    let foio_dir = format!("{}/.foio", home_dir);
    let foio_path = Path::new(&foio_dir);

    if !foio_path.exists() {
        init_pipeline(log);
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

pub fn generate_foio_script(log: bool) -> String {
    let script = r#"#!/bin/bash
    
if [ $# -eq 0 ]; then
    echo "No arguments provided. Usage: $0 <your_argument>"
    exit 1
fi

fileType="$1"

cd ~
cd .foio

if [ "$fileType" = "page" ]; then
    file="page.md"
elif [ "$fileType" = "calendar" ]; then
    file="calendar.md"
else
    file="pages/$fileType.md"
fi
"#;
    let editor = config_decode(log);
    let file_script = format!(
        r#"
if [ -e "$file" ]; then
    {} "$file"
else
    echo "File $file does not exist."
fi
"#,
        editor
    );

    let combined_script = script.to_string() + &file_script;
    return combined_script;
}

pub fn write_foio_script(log: bool) {
    let home_dir = get_home_dir();
    let foioscript = generate_foio_script(log);
    let foio_dir = format!("{}/.foio", home_dir);
    let foioscript_path = PathBuf::from(foio_dir).join("foioscript.sh");
    if foioscript_path.exists() {
        if let Err(err) = fs::write(&foioscript_path, "") {
            panic!("Error truncating page.md: {:?}", err);
        } else {
            if log {
                println!("Truncated foioscript.sh at: {:?}", foioscript_path);
            }
        }
        if let Err(err) = fs::write(&foioscript_path, foioscript) {
            panic!("Error writing the foioscript.sh {:?}", err);
        } else {
            if log {
                println!("Created the foioscript.sh file");
            }
        }
    } else {
        // create page.md
        match fs::File::create(&foioscript_path) {
            Ok(_) => {
                if log {
                    println!("created page.md at: {:?}", foioscript_path);
                }
                if let Err(err) = fs::write(&foioscript_path, foioscript) {
                    panic!("Error writing the foioscript.sh {:?}", err);
                } else {
                    if log {
                        println!("Created the foioscript.sh file");
                    }
                }
            }
            Err(err) => {
                panic!("Error creating page.md: {:?}", err);
            }
        }
    }
}

pub fn change_permission_to_foio_script(log: bool) {
    let home_dir = get_home_dir();
    let foioscript_path = format!("{}/.foio/foioscript.sh", home_dir);
    let status = Command::new("chmod")
        .arg("+x")
        .arg(foioscript_path)
        .status()
        .expect("Failed to execute the chmod command");
    if status.success() {
        if log {
            println!("Permissions changed successfully");
        }
    } else {
        if log {
            println!("Failed to change permissions");
        }
    }
}


pub fn setup_pipeline(log: bool) {
    write_foio_script(log);
    change_permission_to_foio_script(log);

}