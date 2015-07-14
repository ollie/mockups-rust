//! Just a bunch of cross-module functions.

use std::io;

use std::fs;
use std::fs::{
    File
};

use std::path::Path;

pub fn create_dir(path: &Path) {
    if is_dir(path) {
        return;
    }

    let _ = fs::create_dir(path)
        .ok()
        .expect(&format!("Cannot create directory: {}", &path.to_str().unwrap()));
}

pub fn create_file(path: &Path) -> io::Result<File> {
    File::create(path)
}

pub fn is_dir(file_path: &Path) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_)       => false
    }
}

pub fn is_file(file_path: &Path) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => metadata.is_file(),
        Err(_)       => false
    }
}
