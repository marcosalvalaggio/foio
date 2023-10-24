use chrono::Local;
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use std::io::Write;
use super::init::get_home_dir;

pub fn update_pages() {
    let home_dir: String = get_home_dir();
    let foio_dir: String = format!("{}/.foio", home_dir);
    let page_path = format!("{}/.foio/page.md", home_dir);
    let current_date = Local::now();
    let formatted_date: String = current_date.format("%m-%d-%y").to_string();
    let date_file_name: String = format!("pages/{}.md", formatted_date);
    let date_page_path: PathBuf = PathBuf::from(foio_dir.clone()).join(date_file_name);

    // Open the date_page_path file in write mode, creating it if it doesn't exist
    let mut saved_page_file = match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Truncate if the file exists
        .open(&date_page_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Error opening the destination file: {:?}", err);
            }
        };

    // Copy from page.md to destination
    let mut page_file = fs::File::open(&page_path).expect("Error opening page.md");
    let mut buffer = Vec::new();
    page_file.read_to_end(&mut buffer).expect("Error reading page.md");
    saved_page_file.write_all(&buffer).expect("Error writing to the destination file");
}
