use clap::{command, Arg};
use std::{fs, path::PathBuf, result::Result};
use termimad::MadSkin;
use walkdir::WalkDir;

fn main() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".h-files");
    if verify_dir(&storage_dir) {
        let matches = command!().arg(Arg::new("file")).get_matches();
        if let Some(f) = matches.get_one::<String>("file") {
            let mut file_path = storage_dir.clone();
            file_path.push(f);
            file_path.set_extension("md");

            if file_path.exists() {
                let text = fs::read_to_string(file_path).unwrap();
                let skin = MadSkin::default();
                skin.print_text(&text);
            } else {
                println!("No file for: {f}");
            }
        } else {
            println!("Existing files:");
            list_files();
        }
    } else {
        println!("Could not make storage directory");
        println!("{}", storage_dir.display());
    }
}

fn verify_dir(dir: &PathBuf) -> bool {
    if dir.exists() {
        true
    } else {
        fs::create_dir_all(dir).is_ok()
    }
}

pub fn list_files() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".h-files");
    for entry in WalkDir::new(storage_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.path().is_file() {
            println!("{}", entry.path().file_stem().unwrap().to_str().unwrap());
        }
    }
}
