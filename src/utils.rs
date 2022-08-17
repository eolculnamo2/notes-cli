use dirs::home_dir;
use std::fs;

pub fn get_base_path() -> String {
     match home_dir() {
        Some(d) => format!("{}/rob-utils/notes-cli", d.to_str().unwrap()),
        None => panic!("Unable to find OS home dir"),
    }
}

pub fn handle_missing_base() -> Result<fs::ReadDir, Result<(), std::io::Error>> {
    fs::read_dir(get_base_path().as_str())
        .or_else(|_| {
            println!("Base path does not exist. Adding path...");
            Err(fs::create_dir_all(get_base_path().as_str())) 
    }) 
}


