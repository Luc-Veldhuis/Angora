// create a tmpfs to make our fuzzing faster
// only for :
// inputs directory and .cur_input, .socket file

use std::{fs, path::Path};

pub fn create_tmpfs_dir(target: &Path) {
    if !target.exists() {
        fs::create_dir(&target).unwrap();
    }
}

pub fn clear_tmpfs_dir(target: &Path) {
    if target.exists() {
        fs::remove_file(target).unwrap();
    }
}
