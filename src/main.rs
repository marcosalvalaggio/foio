mod command;

fn main() {
    println!("Hello, seaman!");
    command::init::init_pipeline();
    let editor = command::setup::config_decode();
    println!("{}", editor);
}
