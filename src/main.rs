use std::fs;
use clap::{Arg, Command};
use colored::*;

mod utils;

fn main() {
    let matches = Command::new("Notes CLI")
        .arg(
            Arg::new("new-dir")
            .short('n')
            .long("new-dir")
            .takes_value(true)
            .value_name("new_dir")
            .help("Creates a new directory")
            )
        .arg(
            Arg::new("write")
            .short('w')
            .long("write")
            .takes_value(true)
            .value_name("path")
            .help("Write to new or existing file")
        )
        .arg(
            Arg::new("ls")
            .short('l')
            .long("ls")
            .takes_value(true)
            .required(false)
            .value_name("path")
            .help("Show contents of directory")
        )
        .get_matches();

    let _ = utils::handle_missing_base();

    if let Some(new_path) = matches.get_one::<String>("new-dir") {
       let new_dir = utils::get_base_path() + "/" + new_path;  
        match fs::create_dir(new_dir) {
            Ok(_) => println!("Directory created"),
            Err(e) => println!("Failed to create directory: {}", e),
       }
    }

    if let Some(file_path) = matches.get_one::<String>("write") {
        if file_path.ends_with(".txt") == false {
            panic!("Must end with .txt extension!");
        }
        let file_path = utils::get_base_path() + "/" + file_path;
        std::process::Command::new("vim")
        .arg(file_path)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
    }

    if let Some(p) = matches.get_one::<String>("ls") {
        //let path = match p {
        //    Some(p) => utils::get_base_path() + "/" + p,
        //    None => utils::get_base_path(),
        //};
        let path = utils::get_base_path() + "/" + p;
        let contents = fs::read_dir(&path).expect(&("Failed to read path".to_owned() + &path));
        
        for c in contents {
            c.iter()
             .for_each(|item| {
                println!("{}", item.clone()
                            .path()
                            .display())
                });
            println!("{}", c
                     .expect("Invalid path")
                     .path()
                     .file_name()
                     .expect("Failed to get file name")
                     .to_str()
                     .expect("Failed to parse string")
                     .cyan()
                     .bold());
        }
    }
}

