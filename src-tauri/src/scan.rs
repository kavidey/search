use scan_dir::{ScanDir, Error};
use std::path::PathBuf;

pub fn index_directory(root: &str, file_handler: impl Fn(String, PathBuf), error_handler: fn(Error)) {
    let walk_result = ScanDir::files().walk(root, |iter| {
        for (entry, name) in iter {
            file_handler(name, entry.path());
        }
    });
    if let Err(errors) = walk_result {
        for e in errors {
            error_handler(e)
        }
    }
}