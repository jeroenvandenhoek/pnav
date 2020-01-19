use super::input;
use std::error::Error;
use std::fs;

pub struct Program {
    input: input::Input,
}
impl Program {
    pub fn run(input: input::Input) -> Result<(), Box<dyn Error>>{
        let program = Program{
            input: input
        };
        match program.when_project_code() {
            Ok(_) => (),
            Err(_) => panic!("\n\nprogram.when_project_code() failed")
        };

        Ok(())
    }
}

impl Program {
    fn _when_none(&self){
        // for now this function does exactly what the funtion below does
        // this might change in the future
        self.when_project_code();
    }
    fn when_project_code(&self) -> Result<(), Box<dyn Error>>{
        // get project root as text
        let project_root: &str = match &self.input.config_active_project{
            Some(path) => path,
            None => panic!("unable to find project root folder")
        };

        // get client folders from project root
        let client_folders: fs::ReadDir = fs::read_dir(project_root)?;

        // loop through client folders
        let client_root: Vec<fs::DirEntry> = client_folders
            .filter(| c | {
                let c = match c {
                    Ok(value) => value,
                    Err(_) => panic!("can not extract dir entry from result")
                };
                let c = match c.file_name().into_string(){
                    Ok(value) => value,
                    Err(message) => panic!(message)
                };
                if c[0..2] == c {
                    true
                } else {
                    false
                }
            })
            .map(| d | {
                match d {
                    Ok(dir) => dir,
                    Err(message) => panic!(message)
                }
            })
            .collect();

        let client_root = match client_root.first(){
            Some(dir) => dir,
            None => panic!("directory not found")
        };

        println!("\n\n{:?}\n\n", client_root.file_name());

        


        Ok(())

    }
}
