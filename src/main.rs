mod command;
use clap::Parser;

use crate::command::cli::EntityType;

fn main() {
    println!("Hello seaman!");
    let cli = command::cli::FoioArgs::parse_from(std::env::args_os());
    println!("{:?}", cli);

    match &cli.entity_type {
        EntityType::Init(_) => {
            println!("Init Pipeline");
            command::init::init_pipeline(true);
        }
        EntityType::Setup(_) => {
            println!("Setup Pipeline");
            command::setup::setup_pipeline(true);
        }
        EntityType::Open(open_cmd) => {
            println!("Open Pipeline");
            command::open::execute_foio_script(open_cmd.path.clone());
        }
    }
}