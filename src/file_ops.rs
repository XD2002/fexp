use std::fs::{self, remove_file, File};
use std::path::{Path, PathBuf};
use std::process::Command;

pub enum FileType {
    Rust,
    Python,
    Text,
    Image,
    Folder,
    Unknown,
}

pub fn list_directory_contents(path: &Path) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    if let Ok(dir_entries) = fs::read_dir(path) {
        for entry in dir_entries {
            if let Ok(entry) = entry {
                entries.push(path.join(entry.file_name().into_string().unwrap()));
            }
        }
    }
    entries
}

pub fn open_file(path: &Path) {
    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .expect("Failed to open file");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("Failed to open file");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args("/C", "start", path)
        .spawn()
        .expect("Failed to open file");
}

pub fn open_file_with(path: &Path, app: &String) {
    #[cfg(target_os = "linux")]
    Command::new(app)
        .arg(path)
        .spawn()
        .expect("Failed to open file with specified app");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg("-a")
        .arg(app)
        .arg(path)
        .spawn()
        .expect("Failed to open file with specified app");

    #[cfg(target_os = "windows")]
    Command::new(app)
        .arg(path)
        .spawn()
        .expect("Failed to open file with specified app");
}

pub fn get_file_type(path: &Path) -> FileType {
    if path.is_dir(){
        FileType::Folder
    } else if let Some(ext) = path.extension() {
        match ext.to_str() {
            Some("rs") => FileType::Rust,
            Some("py") => FileType::Python,
            Some("txt") => FileType::Text,
            _ => FileType::Unknown,
        }
    } else {
        FileType::Unknown
    }
}

pub fn change_name(path: &Path) {
    todo!("implement");
}

pub fn new_file(dir_path: &Path, file_name: String) {
    let dir_path_str = dir_path.to_str().unwrap();
    let _ = File::create(format!("{}{}", dir_path_str, file_name));
}

pub fn delete_file(path: &Path) {
    let _ = remove_file(path);
}
