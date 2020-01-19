use super::input;
use std::error::Error;
use std::fs;
use std::env;
use std::process;

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
            Err(_) => panic!("\n\nmethod when_project_code() failed")
        };

        Ok(())
    }
}

impl Program {
    fn when_project_code(&self) -> Result<(), Box<dyn Error>>{
        // get project code
        let project_code = self.input.config_active_project.as_ref().expect("cannot find a project code");
        // get project root as text
        let project_root: &str = match &self.input.config_project_root{
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
                if &c[0..3] == &project_code[0..3] {
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


        let client_root: &fs::DirEntry = match client_root.first(){
            Some(dir) => dir,
            None => panic!("directory not found")
        };

        println!("{:?}",client_root.path());
        env::set_current_dir(&client_root.path()).expect("unable to change into directory");

        let output = process::Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process");

        output.stdout;

        Ok(())

    }
}
