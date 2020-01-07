use std::fs;
use std::error::Error;

/// fetches the project folder from the input_root_path that corresponds to the project code.
pub fn get_project_input_folder(project_code: &str, input_root_path: &str) -> Result<fs::DirEntry, Box<dyn Error>> {

    // closure that filters directories to find the directory
    // that corresponds to the project code
    let find_input_dir = | dir: &fs::DirEntry | {
        let dir_name = dir.file_name();

        let dir_name = match dir_name.to_str() {
            Some(name) => name,
            None => ""
        };

        if dir_name.contains(project_code) {true} else {false}
    };

    // fetch the root directory from the input_root_path
    let root_dir = fs::read_dir(input_root_path)?; 

    // cycle through list of directories to find the projects input directory
    let mut input_dir: Vec<fs::DirEntry> = root_dir.map(|x| {
        match x {
            Ok(val) => val,
            Err(_) => panic!("no input directory found")
        }
    }).filter(find_input_dir).collect();

    // extract DirEntry from Vector and return it as result
    match input_dir.pop() {
        Some(val) => Ok(val),
        None => Err(Box::from("no value to pop")),
    }
} 
