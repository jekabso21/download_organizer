use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::io;

pub fn organize(download_dir: &PathBuf) {
    println!("Monitoring download directory: {:?}", download_dir);

    let stop_signal = Arc::new(Mutex::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);
    thread::spawn(move || {
        let mut input = String::new();
        while let Ok(_) = io::stdin().read_line(&mut input) {
            if input.trim() == "stop" {
                let mut stop_signal = stop_signal_clone.lock().unwrap();
                *stop_signal = true;
                break;
            }
            input.clear();
        }
    });
    loop {
        {
            let stop_signal = stop_signal.lock().unwrap();
            if *stop_signal {
                break;
            }
        }

        check_new_files(download_dir);
        thread::sleep(Duration::from_secs(10)); // Check every 10 seconds
    }
}

fn check_new_files(download_dir: &PathBuf) {
    let category_folders: HashSet<&str> = [
        "Documents", "Images", "Videos", "Audio", "Archives", "Code", "Data", "Others", "Unknown"
    ].iter().cloned().collect();

    if let Ok(entries) = fs::read_dir(download_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    handle_new_file(path);
                } else if path.is_dir() {
                    handle_new_folder(path, &category_folders);
                }
            }
        }
    }
}

fn handle_new_file(path: PathBuf) {
    if let Some(extension) = path.extension() {
        let target_dir = match extension.to_str().unwrap().to_lowercase().as_str() {
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "rtf" | "odt" | "csv" => "Documents",

            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "svg" | "ico" | "webp" => "Images",

            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "Videos",

            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "Audio",

            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" => "Archives",

            "html" | "htm" | "css" | "js" | "ts" | "php" | "py" | "rb" | "java" | "c" | "cpp" | "cs" | "go" | "rs" | "swift" | "kt" => "Code",

            "json" | "xml" | "yaml" | "yml" | "sql" | "db" => "Data",

            _ => "Others",
        };

        println!("Moving file {:?} to {}", path, target_dir);
        move_file_to_directory(&path, target_dir);
    } else {
        println!("Moving file {:?} to Others", path);
        move_file_to_directory(&path, "Others");
    }
}

fn handle_new_folder(path: PathBuf, category_folders: &HashSet<&str>) {
    let folder_name = path.file_name().unwrap().to_str().unwrap();

    if category_folders.contains(folder_name) {
        println!("Ignoring category folder {:?}", path);
        return;
    }

    println!("Moving folder {:?} to Unknown", path);
    move_file_to_directory(&path, "Unknown");
}

fn move_file_to_directory(path: &PathBuf, target_dir: &str) {
    let target_path = path.parent().unwrap().join(target_dir);
    if !target_path.exists() {
        fs::create_dir(&target_path).unwrap();
    }

    let new_path = target_path.join(path.file_name().unwrap());

    match fs::rename(path, &new_path) {
        Ok(_) => println!("Moved {:?} to {:?}", path, new_path),
        Err(e) => println!("Failed to move {:?}: {}", path, e),
    }
}
