mod command;
use clap::Parser;

use crate::command::cli::EntityType;

fn main() {
    let cli = command::cli::FoioArgs::parse_from(std::env::args_os());
    // println!("{:?}", cli);

    match &cli.entity_type {
        EntityType::Init(init_cmd) => {
            if let Some(_) = &init_cmd.log_options {
            // if let Some(log_options) = &init_cmd.log_options {
                // println!("init log=true");
                // println!("{}", log_options.log);
                command::init::init_pipeline(true);
            } else {
                // If log_options is None, pass false by default
                // println!("init log=false");
                command::init::init_pipeline(false);
            }
        }
        EntityType::Setup(setup_cmd) => {
            if let Some(_) = &setup_cmd.log_options {
            // if let Some(log_options) = &setup_cmd.log_options {
                // println!("setup log=true");
                // println!("{}", log_options.log);
                command::setup::setup_pipeline(true);
            } else {
                // println!("setup log=false");
                command::setup::setup_pipeline(false);
            }
        }
        EntityType::Open(open_cmd) => {
            command::open::execute_foio_script(open_cmd.path.clone());
            if &open_cmd.path=="page" {
                command::update::update_pages();
            }
        }
    }
}