mod organize;
mod revert;

use std::fs;
use std::io::{Write, Read};
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select, Input};

const PATH_FILE: &str = "download_path.txt";

fn main() {
    loop {
        let download_dir = get_download_dir();

        let options = &["Organize", "Revert", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(options)
            .interact()
            .unwrap();

        match selection {
            0 => organize::organize(&download_dir),
            1 => revert::revert(&download_dir),
            2 => std::process::exit(0),
            _ => unreachable!(),
        }
    }
}

fn get_download_dir() -> PathBuf {
    if fs::metadata(PATH_FILE).is_ok() {
        let mut file = fs::File::open(PATH_FILE).unwrap();
        let mut path = String::new();
        file.read_to_string(&mut path).unwrap();
        PathBuf::from(path)
    } else {
        let path = ask_download_dir();
        let mut file = fs::File::create(PATH_FILE).unwrap();
        file.write_all(path.as_bytes()).unwrap();
        PathBuf::from(path)
    }
}

fn ask_download_dir() -> String {
    let path: String = Input::new()
        .with_prompt("Please enter your download directory")
        .interact_text()
        .unwrap();
    let canonical_path = fs::canonicalize(&path).unwrap();
    canonical_path.to_str().unwrap().to_string()
}
