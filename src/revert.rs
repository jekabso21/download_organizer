use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io;

pub fn revert(download_dir: &PathBuf) {
    println!("Reverting organization in directory: {:?}", download_dir);

    let subdirectories = vec!["PDFs", "Images", "Videos", "Archives", "Music", "Others", "Unknown"];

    let stop_signal = Arc::new(Mutex::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);

    // Listening thread so when user writes stop it will stop the process
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

    // Starting reverting process
    loop {
        {
            let stop_signal = stop_signal.lock().unwrap();
            if *stop_signal {
                break;
            }
        }

        revert_files(download_dir, &subdirectories);
        thread::sleep(Duration::from_secs(10));
    }
}

fn revert_files(download_dir: &PathBuf, subdirectories: &[&str]) {
    for subdir in subdirectories {
        let subdir_path = download_dir.join(subdir);
        if subdir_path.exists() && subdir_path.is_dir() {
            if let Ok(entries) = fs::read_dir(&subdir_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        println!("Moving file/folder {:?} back to {:?}", path, download_dir);
                        move_file_to_directory(&path, download_dir);
                    }
                }
            }
        }
    }
}

fn move_file_to_directory(path: &PathBuf, target_dir: &PathBuf) {
    let new_path = target_dir.join(path.file_name().unwrap());

    match fs::rename(path, &new_path) {
        Ok(_) => println!("Moved {:?} to {:?}", path, new_path),
        Err(e) => println!("Failed to move {:?}: {}", path, e),
    }
}
