use filetime::{set_file_mtime, FileTime};
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// recursively iterate through all directories and return all the paths
pub fn get_all_files_in_directory<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = path.as_ref();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut get_all_files_in_directory(path));
        } else {
            files.push(path);
        }
    }

    files
}

// return all files having given criteria
// crieteria can be like ["*", "*.rs", "log*.txt",log.*]
// * means match all characters

pub fn get_files_in_directory_with_criteria<P: AsRef<Path>>(
    path: P,
    criteria: &[String],
) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let paths = get_all_files_in_directory(path);

    for path in paths {
        if is_path_matched(&path, criteria) {
            files.push(path);
        }
    }

    files
}

fn is_path_matched(path: &PathBuf, criteria: &[String]) -> bool {
    for c in criteria {
        let is_match = match c.as_str() {
            // all
            "*" => true,
            // same name or same ext
            x if c.starts_with('*')
                && path.to_str().unwrap().ends_with(x.trim_start_matches('*')) =>
            {
                true
            }
            x if c.ends_with('*')
                && path.to_str().unwrap().starts_with(x.trim_end_matches('*')) =>
            {
                true
            }
            // belongs to dir
            x if c.ends_with("/") && path.to_str().unwrap().contains(x.trim_end_matches("/")) => {
                true
            }
            x => path.file_name().unwrap() == x,
        };

        if is_match {
            return true;
        }
    }
    false
}

// get the last modified date of files given in params
pub fn get_last_modified_of_files<P: AsRef<Path>>(paths: &[P]) -> Vec<SystemTime> {
    let mut modifieds = Vec::new();

    for path in paths {
        let metadata = match fs::metadata(path) {
            Ok(data) => data,
            Err(_) => continue,
        };
        let modified = metadata.modified().unwrap();
        modifieds.push(modified);
    }

    modifieds
}

// update the last modified date of the given files
pub fn update_last_modified_of_files<P: AsRef<Path>>(paths: Vec<P>) {
    for path in paths {
        set_file_mtime(path, FileTime::now()).unwrap();
    }
}

pub fn did_other_files_changed<P: AsRef<Path>>(main_comparor: P, other_file_paths: Vec<P>) -> bool {
    let main_modified_time = get_last_modified_of_files(&[main_comparor])[0];

    for second_path in other_file_paths {
        let second_modified_time = get_last_modified_of_files(&[second_path])[0];
        if second_modified_time > main_modified_time {
            return true;
        }
    }

    false
}
