use chrono::Local;
use std::path::PathBuf;
use std::fs;
use super::init::get_home_dir;

pub fn update_pages() {
    let home_dir: String = get_home_dir();
    let foio_dir: String = format!("{}/.foio", home_dir);
    let page_path = format!("{}/.foio/page.md", home_dir);
    let current_date = Local::now();
    let formatted_date: String = current_date.format("%m-%d-%y").to_string();
    let date_file_name: String = format!("{}.md", formatted_date);
    // let page_path = PathBuf::from(foio_dir.clone()).join("page.md");
    let date_page_path: PathBuf = PathBuf::from(foio_dir.clone()).join(date_file_name);
    if date_page_path.exists() {
        if let Err(err) = fs::write(&date_page_path, "") {
            panic!("Error truncating page.md: {:?}", err);
        }
    } else {
        match fs::File::create(&date_page_path) {
            Ok(_) => {}
            Err(err) => {
                panic!("Error creating page.md: {:?}", err);
            }
        }
    }

    // copy from page to destination
    let mut page_file = File::open(page_path.as_str());

}