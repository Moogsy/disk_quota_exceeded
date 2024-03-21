use std::path::Path;
use std::fs::{self, ReadDir};
use std::env;

pub fn get_starting_path_name() -> String {
    let current_path: Option<String> = env::args().next();
    let maybe_provided: Option<String> = env::args().next();

    if let Some(path) = maybe_provided {
        path
    } else {
        current_path.unwrap()
    }
}

pub fn check_initial_path(initial_path: &Path) -> Result<ReadDir, String> {
    if !initial_path.is_dir() {
        let err_msg = format!(
            "Provided path `{}` is not a directory !", 
            initial_path.to_string_lossy()
        );
        return Err(err_msg);
    }

    fs::read_dir(initial_path).map_err(|err| err.to_string())
}


