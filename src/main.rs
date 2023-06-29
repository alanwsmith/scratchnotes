use dirs;
use std::fs;

fn main() {
    let mut default_path = dirs::home_dir().unwrap();
    default_path.push(".h-files");
    default_path.push("default.txt");
    if default_path.exists() {
        let text = fs::read_to_string(default_path).unwrap();
        println!("{}", text);
    } else {
        println!("No file at: {}", default_path.display());
    }
}
