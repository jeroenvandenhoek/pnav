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
        Input.parse_flags();

        println!("collected arguments: {:?}", &Input.arguments);
        println!("collected flags_general: {:?}", &Input.flags_general);
        println!("collected flags_targeting_project: {:?}",
            &Input.flags_targeting_project);
        println!("collected flags_targeting_production: {:?}",
            &Input.flags_targeting_production);

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

        // combine possible "new" argument with the following argument
        // step 1: check at which index "add" or "new" is
        let mut index_of_add_or_new: Option<usize> = args.iter().rposition(| x | {
            x.to_lowercase() == "new" || x.to_lowercase() == "add"
        });

        // if add or new is present in the vector:
        // create a group with the argument that follows (if that)
        let grouped_add_or_new: Option<String> = match &index_of_add_or_new {
            Some(index) => {
                if (index+1 <= args.len()){
                    Some(format!("{} {}", args[*index], args[*index+1]))
                }else{
                    None
                }
            },
            None => None
        };

        // if grouped_add_or_new is not none:
        // insert grouped.. into args and
        // remove the separate arguments (new, and following)
        // otherwise don't do anything
        if (grouped_add_or_new != None){
            let index = index_of_add_or_new.expect("No index for 'add' or 'new' found");
            args.remove(index+1);
            args.remove(index);
            args.insert(0,grouped_add_or_new.expect("No 'add' or 'new' group found."));
        }

        // if there are no arguments; return None,
        // otherwise return the arguments in Some
        self.arguments = match args.len() {
            0 => None,
            _ => Some(args)
        };
    }
    fn parse_flags(&mut self) {
        // collect arguments and filter out everything but the flags
        let args = args();
        let flags: Vec<String> = args.filter(|x| x.starts_with("-")).collect();

        let long_flags: Vec<String> = flags.iter().filter(| x | x.starts_with("--")).collect();
        println!("\n\nthis is where you left it JJ!!!!!\n\n");
    }
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


