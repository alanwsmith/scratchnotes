use clap::Parser;
use dialoguer::Confirm;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
struct Args {
    #[arg(short, long)]
    delete: Option<String>,
    file: Option<String>,
}

fn main() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".scratchnotes/notes");
    verify_dir(&storage_dir);
    let args = Args::parse();
    match (args.delete, args.file) {
        (Some(name), None) => delete_file(storage_dir, name),
        (None, Some(name)) => edit_file(storage_dir, name),
        (None, None) => list_files(storage_dir),
        _ => {
            println!("Invalid command. You can only pass one option");
        }
    }
}

fn ask_to_make_file(file_path: PathBuf) {
    match Confirm::new()
        .with_prompt("That file doesn't exist. Do you want to make it?")
        .interact()
    {
        Ok(status) => {
            if status == true {
                let _ = edit::edit_file(file_path);
            } else {
                println!("ok, no file created");
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}

fn delete_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("md");
    if file_path.exists() {
        match fs::remove_file(file_path) {
            Ok(_) => {
                println!("Deleted: {}", name.as_str());
            }
            Err(_) => {
                println!("Could not delete: {}", name.as_str());
            }
        }
    } else {
        println!("{} is already gone", name.as_str());
    }
}

fn edit_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("md");
    if file_path.exists() {
        let _ = edit::edit_file(file_path);
    } else {
        ask_to_make_file(file_path);
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
    println!("-----------------------------------------");
    if &entries.len() > &0 {
        println!("Your notes:");
        entries
            .iter()
            .for_each(|entry| println!("{}", entry.path().file_stem().unwrap().to_str().unwrap()));
    } else {
        println!("You don't have any notes. Make one with:");
        println!("");
        println!("    s -n notename");
        println!("");
    }
    println!("-----------------------------------------");
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
