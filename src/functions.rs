// Description: file containing public functions 

use clap::ArgMatches;
use std::path::PathBuf;

pub fn get_dir(arguments: &ArgMatches) -> Option<PathBuf> {
    // Get the path string from arguments
    let arg = arguments.value_of("INPUT");

    // Create PathBuf if path string exists
    let path;
    match arg {
        Some(p) => path = PathBuf::from(p),
        None => return None
    }

    // If the path exists
    if path.exists() {
        // Get absolute link, tracing intermediate symbolic links
        let path_canon = path.canonicalize().unwrap();
        // If the path is already a directory, return it
        if path_canon.is_dir() {
            return Some(path_canon)
        } else if path_canon.is_file() {
            // Get directory from path
            let fname = path_canon.file_name().unwrap().to_str().unwrap();
            let path_dir = PathBuf::from(path_canon.to_str().unwrap().strip_suffix(fname).unwrap());
            if path_dir.is_dir() {
                return Some(path_dir);
            }
        }
    } else {
        return None
    }


    // Otherwise,

    Some(path)

}

// pub fn get_files

// pub fn get_index