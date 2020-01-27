use std::error::Error;
use std::env::args;
use std::fs;
use dirs;

#[derive(Debug)]
pub struct Input{
    pub arguments: Option<Vec<String>>,
    pub arguments_contained_project_code: bool,
    pub flags_general: Option<Vec<char>>,
    pub flags_targeting_project: Option<Vec<char>>,
    pub flags_targeting_production: Option<Vec<char>>,
    pub config_project_root: Option<String>,
    pub config_production_roots: Option<Vec<String>>,
    pub config_active_project: Option<String>,
    pub config_company_name: Option<String>
}

impl<'a> Input{
    pub fn get() -> Input{
        let mut input = Input{
            arguments: None,
            arguments_contained_project_code: false,
            flags_general: None,
            flags_targeting_project: None,
            flags_targeting_production: None,
            config_project_root: None,
            config_production_roots: None,
            config_active_project: None,
            config_company_name: None
        }; 


        input.parse_args();
        input.parse_flags();
        input.get_config();
        input.parse_config();
        match input.write_config() {
            Ok(_)=> (),
            Err(_) => panic!("unable to write to .pnavrc")
        };


        input
    }
}

// top level methods for Input
impl<'a> Input {
    fn parse_args(&mut self) {
        // collect arguments and filter out all the flags
        let args = args();
        let mut args: Vec<String> = args.filter(|x| !x.starts_with("-")).collect();

        // remove the path from the arguments
        args.remove(0);

        // combine possible "new" argument with the following argument
        // step 1: check at which index "add" or "new" is
        let index_of_add_or_new: Option<usize> = args.iter().rposition(| x | {
            x.to_lowercase() == "new" || x.to_lowercase() == "add"
        });

        // if add or new is present in the vector:
        // create a group with the argument that follows (if that)
        let grouped_add_or_new: Option<String> = match &index_of_add_or_new {
            Some(index) => {
                if index+1 <= args.len(){
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
        if grouped_add_or_new != None {
            let index = index_of_add_or_new.expect("No index for 'add' or 'new' found");
            args.remove(index+1);
            args.remove(index);
            args.insert(0,grouped_add_or_new.expect("No 'add' or 'new' group found."));
        }

        // if project code was entered
        // set active project to it (but also keep it in the args vector
        // for program interpretor)
        &args.iter().for_each(| a | {
            if a.len() == 6 {
                let mut is_valid: bool = true;
                a.chars().for_each(| c | {
                    if !(c.is_numeric() && is_valid) {
                        is_valid = false
                    }
                });
                
                if is_valid {
                    self.config_active_project = Some(String::from(a));
                    self.arguments_contained_project_code = true;
                }
            }
        });

        // if there are no arguments; return None,
        // otherwise return the arguments in Some
        self.arguments = match args.len() {
            0 => None,
            _ => Some(args)
        };
    }
    fn parse_flags(&mut self) -> Option<()>{
        // collect arguments and filter out everything but the flags
        let args = args();
        let flags: Vec<String> = args
            .filter(|x| x.starts_with("-") && x != "--")
            .collect();

        // stop this function if no flags are given by the user
        if flags.len() == 0 {return None};

        // get long flags
        let long_flags: Vec<&String> = flags
            .iter()
            .filter(| x | x.starts_with("--"))
            .collect();

        // get short flag in raw form;
        // meaning all short flags that follow a single hyphen,
        // within the same argument
        let short_flags_raw: Vec<&String> = flags
            .iter()
            .filter(| x | !x.starts_with("--"))
            .collect();

        // sepparate the shortflags to single chars
        let mut short_flags: Vec<char> = Vec::new();
        short_flags_raw
            .iter()
            .for_each(| x | {
                let without_hyphen = x.replace('-', "");
                without_hyphen
                    .chars()
                    .for_each(| x | {
                        short_flags.push(x);
                    })
            });

        // if there are long flags present;
        // convert long flags to short flags
        // and add them to list of short flags
        if long_flags.len() != 0 {
            long_flags
                .iter()
                .for_each(| x |{
                    short_flags.push(match &x.to_lowercase()[..] {
                        "--list" => 'l',
                        "--help" => 'h',
                        "--client" => 'c',
                        "--supplier" => 's',
                        "--me" => 'm',
                        "--my-company" => 'm',
                        "--assets" => 'a',
                        "--projects" => 'p',
                        "--delivery" => 'd',
                        _ => 'X',  
                    })
                });
        };

        // add short flags to corresponding categories
        let mut general: Vec<char> = Vec::new();
        let mut project: Vec<char> = Vec::new();
        let mut production: Vec<char> = Vec::new();
        short_flags.iter().for_each(| x | {
            match *x {
                'l' => general.push('l'),
                'h' => general.push('h'),
                'c' => project.push('c'),
                's' => project.push('s'),
                'm' => project.push('m'),
                'a' => production.push('a'),
                'p' => production.push('p'),
                'd' => production.push('d'),
                _ => ()
            }
        });

        // write three flag categories to corresponding Struct fields as options
        match general.len() {
            0 => self.flags_general = None,
            _ => self.flags_general = Some(general)
        }
        match project.len() {
            0 => self.flags_targeting_project = None,
            _ => self.flags_targeting_project = Some(project)
        }
        match production.len() {
            0 => self.flags_targeting_production = None,
            _ => self.flags_targeting_production = Some(production)
        }
        Some(())
    }
    fn parse_config(&mut self) {
        let config: String = self.get_config().expect("pnavrc not found");

        self.process_config_production_roots(&config).unwrap();

        config.lines().for_each(| l | {
            self.process_config_active_project(l).unwrap();
            self.process_config_projects_root_dir(l).unwrap();
        });
    }
    fn get_config(&mut self) -> Option<String>{
        let home_dir: String = String::from(dirs::home_dir()
            .expect("could not get home_dir for current operating system")
            .to_str()
            .expect("could not create string from path_buf"));

        let pnavrc: Option<String> =
            match fs::read_to_string(format!("{}/.pnavrc",home_dir)){
                Ok(content) => Some(content),
                Err(_) => match self.write_config(){
                    Ok(content) => Some(content),
                    Err(message) => {
                        panic!("{}",message);
                    }
                },
        };
        pnavrc
    }
    fn write_config(&self) -> Result<String, Box<dyn Error>>{
        // get home directory
        let home_dir = match dirs::home_dir() {
            Some(path_buf) => String::from(path_buf
                .as_path()
                .to_str()
                .expect("home directory not found")),
            None => panic!("Home directory not found")
        };

        // get project root directory as text from struct if it exists.
        // if it doesn't; define one to write to the .pnavrc file
        let project_root: String = match &self.config_project_root {
            Some(text) => format!("projects_root_dir = {}", text),
            None => format!("projects_root_dir = {}/pnav_project_root_for_testing", home_dir)
        };

        // get active project code as &str from struct if it exists.
        // if it doesn't: set an empty &str
        let active_project: String = match self.config_active_project.as_ref() {
            Some(project) => format!("active_project = {}",project),
            None => String::from(""),
        };

        // get name of user's company if it exist.
        // if it doesn't; set it to a default value.
        let company_name: &str = match &self.config_company_name {
            Some(name) => name,
            None => "company_name = my_company"
        };

        // get the production root directories as a vector of text from struct if it exists.
        // if it doesn't; set empty string
        let production_roots: String = match self.config_production_roots.as_ref() {
            Some(roots) => {
                let mut roots_as_str: String = String::from("[production-roots]");
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

        // combine content to write to .pnavrc
        let content: String = format!("{}\n\n{}\n\n{}\n\n{}",
            project_root,
            active_project,
            company_name,
            production_roots
            );

        // clean up content formatting
        let mut content_clean = String::new();
        content.lines().for_each(| l | {
            if l != "" {
                content_clean = format!("{}\n{}", content_clean, l);
            };
        });


        fs::write(format!("{}/.pnavrc",home_dir), content_clean)?;
        Ok(String::from("hi"))
    }
}

// process config lines from .pnavrc
impl<'a> Input {
    fn process_config_active_project(&mut self, line: &str)->Result<(), Box<dyn Error>>{
        // when line is active project and the code is of valid patern
        // extract the project code and send it to corresponding struct field
        match line.to_lowercase().replace(" ", "").contains("active_project=") {
            true => {
                let mut project: Option<String> = None;
                line
                .to_lowercase()
                .replace(" ", "")
                .split("=")
                .for_each(| x | {
                    if !x.contains("active_project") && x.len() == 6 {
                        project = Some(String::from(x));
                    }
                });
                if !self.arguments_contained_project_code {
                    self.config_active_project = project;
                }
            },
            false => (),
        };
        Ok(())
    }
    fn process_config_projects_root_dir(&mut self, line: &str) -> Result<(), Box<dyn Error>>{
        // when line is project_root_dir
        // extract the dir and send it to the corresponding struct field
        match line.contains("projects_root_dir"){
            true => {
                line
                .split(" = ")
                .for_each(| x | {
                    if !x.contains("projects_root_dir") {
                        let projects_root = String::from(x);
                        let mut with_spaces: String = String::new();
                        let mut iter_count = 0;
                        projects_root.split("\\").for_each(| x | {
                            if iter_count == 0 {
                                with_spaces = format!("{}", x);
                            } else {
                                with_spaces = format!("{} {}", with_spaces, x);
                            }
                            iter_count += 1;
                        });
                        self.config_project_root = Some(with_spaces);
                    }
                });
            },
            false => ()
        };
        Ok(())
    }
    fn process_config_production_roots(&mut self, config: &str)->Result<(), Box<dyn Error>>{
        let mut config: Vec<&str> = config.split("[production-roots]").collect();
        if config.len() == 2 {
            let config: Vec<&str> = config.remove(1).split("\n").collect();
            let mut config: std::slice::Iter<&str> = config.iter();
            let mut production_roots: Vec<String> = vec![];
            for _ in 0..config.len(){
                let line: &str = config.next().unwrap(); // option should return Some, otherwise the loop wouldn't have gotten here 
                if line != "" {
                    production_roots.push(String::from(line));
                }
            };
            if production_roots.len() != 0{
                self.config_production_roots = Some(production_roots);
            };
        } else {
            self.config_production_roots = None;

        }
        Ok(())
    }

}


