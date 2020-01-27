
use std::fs;
use ansi_term;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Green;

pub struct Info {}

impl Info {
    pub fn list_clients(project_root: fs::ReadDir){
        // print client and folder names
        let mut client_code_and_name: Vec<String> = vec![];
        project_root.for_each(| d |{
            // get name of item in directory
            let dir: fs::DirEntry = d.unwrap();
            let path_name: std::path::PathBuf = dir.path();
            let path_name: &str = path_name.to_str().unwrap();
            let mut path_name: Vec<&str> = path_name.split("/").collect();
            let item_name: String = String::from(path_name.remove(path_name.len()-1));

            // check if item is a client directory
            // if it is, store it for sorting
            if item_name.len() > 7 {
                let mut item_name_chars: std::str::Chars = item_name.chars();

                // check if first three chars are numeric 
                let mut are_numeric: bool = false;
                for _ in 0..2 {
                    let current: char = item_name_chars.next().unwrap();
                    are_numeric = current.is_numeric();
                };

                // print if item is client directory
                if are_numeric {
                    client_code_and_name.push(String::from(&item_name));
                }
            }
        });
        // sort client codes
        client_code_and_name.sort();
        client_code_and_name.iter().for_each(| d | {
            let code: &str = &d[0..3];
            let name: &str = &d[6..d.len()];

            println!("{} - {}",Green.paint(code), Yellow.paint(name));
        })
    }
}
