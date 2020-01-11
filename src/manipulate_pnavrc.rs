use std::fs;
use std::fs::File;
use std::error::Error;

#[derive(Debug)]
pub struct Settings <'a> {
    pnavrc_file_path: &'a str,
    pub path_to_input_database: &'a str,
    pub current_project_code: &'a str,
}

impl<'a> Settings<'a> {
    /// creates a new instance of the settings struct and populates it with default values
    pub fn new (project_code: &'a str) -> Settings<'a> {
        Self {
            pnavrc_file_path: "/root/.pnavrc",
            path_to_input_database: "/root/pnav_project_root_for_testing",
            current_project_code: project_code
        }
    }

    /// creates an empty file called .pnavrc in the root directory
    pub fn create_empty_pnavrc() {
        match fs::File::create("/root/.pnavrc") {
            Ok(file) => file,
            Err(message) => panic!(message)
        };
    }

    /// reads .pnavrc and stores the content in the Settings struct
    pub fn read_pnavrc(&mut self) -> Result<File, Box<Error>> {
        let file = File::open("/root/.pnavrc")?;

        Ok(file)
    }
}

pub fn test_program() {
    let mut Settings = Settings::new("002001");
    Settings::create_empty_pnavrc();
    Settings.read_pnavrc();

}
