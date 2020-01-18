use std::error::Error;
use std::env;
use std::env::args;
use std::fs;
use dirs;

pub struct Input{
    arguments: Option<Vec<String>>,
    flags_general: Option<Vec<String>>,
    flags_targeting_project: Option<Vec<String>>,
    flags_targeting_production: Option<Vec<String>>,
    config_project_root: Option<String>,
    config_production_roots: Option<Vec<String>>,
    config_active_project: Option<String>,
    config_company_name: Option<String>
}

impl<'a> Input{
    pub fn get() -> Input{
        let mut Input = Input{
            arguments: None,
            flags_general: None,
            flags_targeting_project: None,
            flags_targeting_production: None,
            config_project_root: None,
            config_production_roots: None,
            config_active_project: None,
            config_company_name: None
        }; 

        Input.parse_args();
        println!("collected arguments: {:?}", &Input.arguments);

        Input.get_config();

        Input
    }
}
impl<'a> Input {
    fn parse_args(&mut self) {
        // collect arguments and filter out all the flags
        let args = args();
        let mut args: Vec<String> = args.filter(|x| !x.starts_with("-")).collect();

        // remove the path from the arguments
        args.remove(0);

        // if there are no arguments; return None,
        // otherwise return the arguments in Some
        self.arguments = match args.len() {
            0 => None,
            _ => Some(args)
        };
    }
    fn parse_flags(&mut self) {}
    fn parse_config(&mut self) {}
    fn get_config(&mut self) -> Option<String>{
        let pnavrc: Option<String> = match fs::read_to_string("~/.pnavrc"){
            Ok(content) => Some(content),
            Err(_) => match self.write_config(){
                Ok(content) => Some(content),
                Err(message) => {
                    //println!("{}","unable to write to .pnavrc");
                    println!("{}",message);
                    None
                }
            },
        };

        pnavrc
    }
    fn write_config(&self) -> Result<String, Box<Error>>{
        // get home directory
        let home_dir = match dirs::home_dir() {
            Some(path_buf) => match path_buf.as_path().to_str(){
                Some(path) => String::from(path),
                None => String::new()
            },
            None => panic!("Home directory not found")
        };

        // get project root directory as text from struct if it exists.
        // if it doesn't; define one to write to the .pnavrc file
        let project_root: String = match &self.config_project_root {
            Some(text) => String::from(text),
            None => format!("projects_root_dir = {}/pnav_project_root_for_testing", home_dir)
        };

        // get the production root directories as a vector of text from struct if it exists.
        // if it doesn't; set empty string
        let production_roots: String = match &self.config_production_roots {
            Some(roots) => {
                let mut roots_as_str: String = String::from("[production_roots]");
                let _roots: Vec<_> = roots
                    .iter()
                    .map(
                        |x| {
                            roots_as_str = format!("{}\n{}",roots_as_str,x)
                        })
                    .collect();
                roots_as_str
            },
            None => String::from("")
        };

        // get active project code as &str from struct if it exists.
        // if it doesn't: set an empty &str
        let active_project: &str = match &self.config_active_project {
            Some(project) => project,
            None => "",
        };

        // get name of user's company if it exist.
        // if it doesn't; set it to a default value.
        let company_name: &str = match &self.config_company_name {
            Some(name) => name,
            None => "company_name = my_company"
        };

        // combine content to write to .pnavrc
        let content: String = format!("{}\n\n{}\n\n{}\n\n{}",
            project_root,
            production_roots,
            active_project,
            company_name
            );

        fs::write(format!("{}/.pnavrc",home_dir), content)?;
        Ok(String::from("hi"))
    }

}


