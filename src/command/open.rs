use super::init::get_home_dir;
use std::process::Command;

// arg = page, calendar, <date>
pub fn execute_foio_script(arg: String) {
    let home_dir = get_home_dir();
    let script_path = format!("{}/.foio/foioscript.sh", home_dir);
    let status = Command::new("sh")
        .arg(script_path)
        .arg(arg.to_string())
        .status()
        .expect("failed");
    if status.success() {
        println!("ok");
    } else {
        panic!("Error opening {:?}", arg);
    }
}
