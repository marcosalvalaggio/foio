mod command;

fn main() {
    println!("Hello, seaman!");
    command::init::init_pipeline(true);
    // let editor = command::setup::config_decode();
    // println!("{}", editor);
    // let script = command::setup::generate_foio_script();
    // println!("{}", script);
    command::setup::write_foio_script(true);
    command::setup::change_permission_to_foio_script(true);
    command::open::execute_foio_script(String::from("10-04-2023"));
}
