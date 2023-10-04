mod command;

fn main() {
    println!("Hello, seaman!");
    command::init::init_pipeline();
    let editor = command::setup::config_decode();
    println!("{}", editor);
    let script = command::setup::generate_foio_script();
    println!("{}", script);
    command::setup::write_foio_script();
    command::setup::change_permission_to_foio_script();
    command::open::execute_foio_script(String::from("page"));
}
