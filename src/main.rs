// use clap::command;
// use clap::Arg;
// use dirs;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
struct Args {
    #[arg(short, long)]
    new_file: Option<String>,
    existing_file: Option<String>,
}

fn main() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".h-files");

    let args = Args::parse();
    match (args.new_file, args.existing_file) {
        (None, Some(name)) => {
            // println!("use exsting file: {}", name);
            show_file(storage_dir, name);
        }
        (Some(name), None) => {
            println!("make a new file: {}", name);
        }
        (Some(_), Some(_)) => {
            println!(
                "Got both a new and existing file. You can only send one. Check --help for details"
            );
        }
        _ => {
            println!("show file list");
        }
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

// fn main() {
//     let mut storage_dir = dirs::home_dir().unwrap();
//     storage_dir.push(".h-files");
//     match verify_dir(&storage_dir) {
//         true => {
//             let matches = command!().arg(Arg::new("file")).get_matches();
//             match matches.get_one::<String>("file") {
//                 Some(f) => {
//                     let mut file_path = storage_dir.clone();
//                     file_path.push(f);
//                     file_path.set_extension("txt");
//                     if file_path.exists() {
//                         let text = fs::read_to_string(file_path).unwrap();
//                         println!("-----------------------------------------");
//                         println!("{}", text);
//                         println!("-----------------------------------------");
//                     } else {
//                         println!("No file for: {}", f);
//                     }
//                 }
//                 None => {
//                     println!("Existing files:");
//                     list_files();
//                 }
//             }
//         }
//         false => {
//             println!("Could not make storage directory");
//             println!("{}", storage_dir.display());
//         }
//     }
// }

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

pub fn list_files() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".h-files");
    for entry in WalkDir::new(storage_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            println!("{}", entry.path().file_stem().unwrap().to_str().unwrap());
        }
    }
}
