mod command;
use clap::Parser;

fn main() {
    println!("Hello seaman!");
    command::init::init_pipeline(true);
    command::setup::setup_pipeline(true);
    command::open::execute_foio_script(String::from("_"));
    let cli = command::cli::FoioArgs::parse();
    println!("{:?}", cli);
}