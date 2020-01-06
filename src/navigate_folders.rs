
use std::fs;

/// fetches the project folder from the input_root_path that corresponds to the project code.
// returns Some()
pub fn get_project_input_folder(project_code: &str, input_root_path: &str) -> fs::DirEntry {
    // closure that filters directories to find the directory
    // that corresponds to the project code
    let find_input_dir = | dir: &fs::DirEntry | {
        let dir_name = dir.file_name();
        let dir_name = match dir_name.to_str() {
            Some(name) => name,
            None => ""
        };
        if dir_name.contains(project_code) {
            true
        } else {
            false
        }
    };

    // fetch the root directory from the input_root_path
    let root_dir = match fs::read_dir(input_root_path) { 
        Ok(val) => val,
        Err(message) => panic!(message)
    };

    // cycle through list of directories to find the projects input directory
    let mut input_dir: Vec<fs::DirEntry> = root_dir.map(|x| {
        match x {
            Ok(val) => val,
            Err(_) => panic!("no input directory found")
        }
    }).filter(find_input_dir).collect();

    match input_dir.pop() {
        Some(val) => val,
        None => panic!("no dir found")
    }
} 
