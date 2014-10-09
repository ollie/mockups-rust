//! Just a bunch of cross-module functions.

use std::io::{
    fs,
    File,
    USER_DIR,
    IoResult,
};

use std::io::fs::PathExtensions;

pub fn create_dir(path: &Path) {
    if path.is_dir() {
        return;
    }

    let _ = fs::mkdir(path, USER_DIR).unwrap();
}

pub fn create_file(path: &Path) -> IoResult<File> {
    File::create(path)
}
