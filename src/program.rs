use super::input;
use std::error::Error;
use std::fs;
use std::env;
use run_script;


pub struct Program {
    input: input::Input,
}
impl Program {
    pub fn run(input: input::Input) -> Result<(), Box<dyn Error>>{
        let program = Program{
            input: input
        };

        program.interpret_input().unwrap();

        Ok(())
    }
    fn interpret_input(&self)->Result<(), Box<dyn Error>>{

        // when user wants to open the project folder
        {
            let project_root: fs::DirEntry = self.get_project_dir()?; 

            match Program::open_path_in_window(&project_root){
                Ok(_) => (),
                Err(error) => panic!("\nunable to open directory\nfound following error:\n{}", error)
            };
        }

        // when user wants to open the production folder
        {
        }

        Ok(())
    }

}

impl Program {
    fn get_project_dir(&self) -> Result<fs::DirEntry, Box<dyn Error>>{
        // get project code
        let project_code = self.input.config_active_project.as_ref().expect("cannot find a project code");
        // get project root as text
        let project_root: &str = match &self.input.config_project_root{
            Some(path) => path,
            None => panic!("unable to find project root folder")
        };

        // remove redundand spaces
        let project_root = project_root.replace("  ", " ");

        // get client folders from project root
        let client_folders: fs::ReadDir = fs::read_dir(project_root)?;

        let client_root: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(client_folders, &project_code[0..3]);
        let project_root: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(fs::read_dir(client_root.path()).expect("unable to read client root directory"), &project_code[0..6]);

        Ok(project_root)
    }
    fn get_production_dir(&self)->Result<fs::DirEntry, Box<dyn Error>>{
        let production_paths: &Vec<String> = self.input.config_production_roots.as_ref().expect("no roots available");
        let production_paths: std::slice::Iter<String> = production_paths.iter();

        let mut prod_dir: Option<fs::DirEntry> = None;

        production_paths.for_each(|p|{
            // check if p is a valid directory 
            let dir: fs::ReadDir = match fs::read_dir(p) {
                Ok(dir_contents) => dir_contents,
                Err(_) => panic!("\ndirectory: {}, not found\n", p)
            };

            // find directory if and return it
            let mut dirs: Vec<fs::DirEntry> = dir.filter(| d | {
                let name_of_dir = d.as_ref()
                    .unwrap() // this shouldn't be an error
                    .file_name()
                    .into_string()
                    .unwrap(); // if this is an error, it's because a folder in the directory doesn't have 6 or more chars
                
                let mut first_six_chars: &str = "";
                if name_of_dir.len() > 5 {
                    first_six_chars = &name_of_dir[0..7];
                };

                if first_six_chars == self.input.config_active_project.as_ref().expect("no active project present"){
                    true
                } else {
                    false
                }
            })
            .map(| d |{
                d.unwrap()
            })
            .collect();

            if dirs.len() != 0 {
                prod_dir = Some(dirs.remove(0));
            }
        });

        match prod_dir {
            Some(dir) => Ok(dir),
            None => panic!("no production directory found")
        }
    }
}

// utilities
impl Program {
    fn find_dir_in_dir_matching_from_start_of_name(read_dir: fs::ReadDir, query: &str) -> fs::DirEntry{
        let mut matching_dirs: Vec<fs::DirEntry> = read_dir
            .filter(| c | {
                let c = match c {
                    Ok(value) => value,
                    Err(_) => panic!("can not extract dir entry from result")
                };
                let c = match c.file_name().into_string(){
                    Ok(value) => value,
                    Err(message) => panic!(message)
                };
                if &c[0..query.len()] == query {
                    true
                } else {
                    false
                }
            })
            .map(| d | {
                let d = match d {
                    Ok(dir) => dir,
                    Err(message) => panic!(message)
                };
                d
            })
            .collect();

        let dir: fs::DirEntry = matching_dirs.remove(0);
        dir
    }
    fn open_path_in_window(dir_entry: &fs::DirEntry) -> Result<(),Box<dyn Error>>{
        // change to directory in process environment
        env::set_current_dir(dir_entry.path()).expect("unable to change into directory");

        // open through script
        run_script!(
            r#"
            open .
            "#
            )?;

        Ok(())
    }

}
