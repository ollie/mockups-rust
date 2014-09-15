//! Just a bunch of cross-module functions.

use std::io::{
    fs,
    File,
    UserDir,
    IoResult,
};

pub fn create_dir(path: &Path) {
    if path.is_dir() {
        return;
    }

    let _ = fs::mkdir(path, UserDir).unwrap();
}

pub fn create_file(path: &Path) -> IoResult<File> {
    File::create(path)
}
