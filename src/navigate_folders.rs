
use std::fs;

/// Gets the project folder that corresponds to the project code.
// returns Some()
pub fn get_project_folder(project_code: &str) -> Option<fs::DirEntry> {
    let path: &str = "/var";

    let filter_closure = | dir_entry: Result<fs::DirEntry, &std::error::Error> | {
        let pcb = &project_code.as_bytes();

        let dir_str = match dir_entry {
            Ok(dir_str) => dir_str,
            Err(_) => panic!("could not read directory name")
        };

        let mut dir_str = dir_str.file_name();
        let dir_str = dir_str.to_str();
        let dirb = match dir_str {
            Some(dir_str) => dir_str.as_bytes(),
            None => panic!("directory name could not be converted to a str"),
        };


        for compare in pcb.iter().zip(dirb) {
            if compare.0 != compare.1 {
                return false
            }
        }
        true
    };


    let mut project_dir: () = fs::read_dir(path)
        .iter()
        .filter(|x| {
            println!("{:?}", x);
            true
        })
        .collect();








    None
}
    
