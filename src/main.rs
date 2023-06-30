// use clap::command;
// use clap::Arg;
// use dirs;
use std::fs;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
struct Args {
    #[arg(short, long)]
    new: Option<String>,
    #[arg(short, long)]
    edit: Option<String>,
    file: Option<String>,
}

fn main() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".speednotes/notes");
    verify_dir(&storage_dir);
    let args = Args::parse();

    // match args.new {
    //     Some(name) => {
    //         make_new_file(storage_dir, name.clone());
    //     }
    //     None => {
    //     }
    // }

    match (args.new, args.edit, args.file) {
        (Some(name), None, None) => {
            make_new_file(storage_dir, name);
        }
        (None, Some(name), None) => edit_file(storage_dir, name),
        (None, None, Some(name)) => show_file(storage_dir, name),
        (None, None, None) => {
            list_files(storage_dir);
        }
        _ => {
            show_help();
        }
    }
}

fn edit_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("txt");
    if file_path.exists() {
        let _ = edit::edit_file(file_path);
    } else {
        println!("No file for: {}", name.as_str());
    }
}

fn show_help() {
    println!("TODO: Show help here");
}

fn make_new_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("txt");
    if file_path.exists() {
        println!("File {} already exists", name.clone());
    } else {
        let _ = edit::edit_file(file_path);
    }
}

fn show_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("txt");
    if file_path.exists() {
        let text = fs::read_to_string(file_path).unwrap();
        println!("-----------------------------------------");
        println!("{}", text);
        println!("-----------------------------------------");
    } else {
        println!("No file for: {}", name.as_str());
    }
}

fn verify_dir(dir: &PathBuf) -> bool {
    if dir.exists() {
        true
    } else {
        match fs::create_dir_all(dir) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

pub fn list_files(storage_dir: PathBuf) {
    let entries = WalkDir::new(storage_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| {
            if e.as_ref().unwrap().path().is_file() {
                e.ok()
            } else {
                None
            }
        })
        .into_iter()
        .collect::<Vec<DirEntry>>();
    if &entries.len() > &0 {
        println!("-----------------------------------------");
        println!("Your notes:");
        entries
            .iter()
            .for_each(|entry| println!("{}", entry.path().file_stem().unwrap().to_str().unwrap()));
        println!("-----------------------------------------");
    } else {
        println!("-----------------------------------------");
        println!("You don't have any notes. Make a new");
        println!("one with:");
        println!("");
        println!("    s -n notename");
        println!("");
        println!("-----------------------------------------");
    }
}
