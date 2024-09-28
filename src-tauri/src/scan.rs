use scan_dir::ScanDir;
use std::path::PathBuf;

pub fn index_directory(root: &str, file_handler: fn(String, PathBuf)) {
    ScanDir::dirs().read(root, |iter| {
        for (entry, name) in iter {
            file_handler(name, entry.path());
        }
    }).unwrap()
}