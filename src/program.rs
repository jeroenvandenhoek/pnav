use super::input;
use std::error::Error;
use std::fs;
use std::env;
use std::process;
use run_script;


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

        // remove redundand spaces
        let project_root = project_root.replace("  ", " ");

        // get client folders from project root
        let client_folders: fs::ReadDir = fs::read_dir(project_root)?;

        let client_root: fs::DirEntry = self.find_dir_in_dir_matching_from_start_of_name(client_folders, &project_code[0..3]);
        let project_root: fs::DirEntry = self.find_dir_in_dir_matching_from_start_of_name(fs::read_dir(client_root.path()).expect("unable to read client root directory"), &project_code[0..6]);

        match self.open_path_in_window(&project_root){
            Ok(_) => (),
            Err(error) => panic!("\nunable to open directory\nfound following error:\n{}", error)
        };

        Ok(())

    }
}

// utilities
impl Program {
    fn find_dir_in_dir_matching_from_start_of_name(&self, read_dir: fs::ReadDir, query: &str) -> fs::DirEntry{
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
    fn open_path_in_window(&self, dir_entry: &fs::DirEntry) -> Result<(),Box<dyn Error>>{
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
