use clap::command;
use clap::Arg;
use dirs;
use std::fs;

fn main() {
    let mut default_path = dirs::home_dir().unwrap();
    default_path.push(".h-files");
    let matches = command!().arg(Arg::new("file")).get_matches();
    match matches.get_one::<String>("file") {
        Some(f) => {
            default_path.push(f);
            default_path.set_extension("txt");
        }
        None => {
            default_path.push("default.txt");
        }
    };
    if default_path.exists() {
        let text = fs::read_to_string(default_path).unwrap();
        println!("");
        println!("{}", text);
    } else {
        println!("No file at: {}", default_path.display());
    }
}
