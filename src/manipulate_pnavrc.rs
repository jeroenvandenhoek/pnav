use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct Settings <'a> {
    pnavrc_file_path: &'a str,
    pub path_to_input_database: &'a mut str,
    pub current_project_code: &'a mut str,
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

    // creates an empty file called .pnavrc in the root directory
    fn create_empty_pnavrc(&self) -> Result<File, &'a str>{
        match fs::File::create(self.pnavrc_file_path) {
            Ok(file) => Ok(file),
            Err(_) => Err("failed to create .pnavrc")
        }
    }

    /// reads .pnavrc and stores the content in the Settings struct
    pub fn read_pnavrc(&mut self) -> Result<(), Box<dyn Error>>{
        // get the .pnavrc file
        // if it doesn't exist; create a new one
        let mut file = match File::open(self.pnavrc_file_path){
            Ok(file) => file,
            Err(_) => {
                self.create_empty_pnavrc()?;
                File::open(self.pnavrc_file_path)?
            }
        };

        // read the contents of the .pnavrc file to a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // sepparate the string into lines for analysis
        let lines: Vec<&str> = contents.lines().collect();

        println!("{}",lines[0]);
        Ok(())
    }

    /// write the changes to .pnavrc
    pub fn write_pnavrc(Settings: Settings) {}
}

pub fn test_program() {
    let Settings = Settings::new("002001");
    Settings.read_pnavrc();

}
