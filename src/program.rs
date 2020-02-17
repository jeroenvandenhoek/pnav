use super::input;
use super::info;
use std::error::Error;
use std::fs;
use std::env;
use ansi_term;
use ansi_term::Colour::Purple;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Green;
use run_script;

enum ArgumentNewType {
    ClientName(String),
    ProjectCode(String),
    Nothing,
}

enum ArgumentType {
    ProjectCode(String),
    ClientCode(String),
    ClientName(String),
    Active, // User asks for a list of active projects
    Current, // User asks what the active project is
    New(ArgumentNewType), // User wants to create someting. 
    Add, // User wants to add a production directory to the .pnavrc file
}

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
        // get project and production dirs
        let project_dir: fs::DirEntry = self.get_project_dir()?; 
        let production_dir: fs::DirEntry = match self.get_production_dir(){
            Some(dir) => dir,
            None => {
                println!("info: this project does not yet have a production directory");
                self.get_project_dir()? // return project directory (find better solution when time is available)
            }
        };

        // shorthands
        let flags_gen: Option<&Vec<char>> = self.input.flags_general.as_ref();
        let flags_proj: Option<&Vec<char>> = self.input.flags_targeting_project.as_ref();
        let flags_prod: Option<&Vec<char>> = self.input.flags_targeting_production.as_ref();

        // interpret flags
        match (flags_gen, flags_proj, flags_prod){
            (Some(gen), _, _) if gen.contains(&'i') => {
                match &self.input.arguments{
                    Some(args) if (args.get(0) != None) => {
                        let arg = args.get(0).unwrap();
                        match Program::argument_type(arg){
                            ArgumentType::ProjectCode(project_code) => println!("\nhere you'll see info about project: {}\n",project_code),
                            ArgumentType::ClientCode(client_code) => println!("\nhere you'll see info about client: {}\n",client_code),
                            ArgumentType::ClientName(_client_name) => (),
                            ArgumentType::Current => self.print_current()?,
                            ArgumentType::Active => self.print_active()?,
                            ArgumentType::New(_input_kind) => (),
                            ArgumentType::Add => (),
                        };
                    },
                    None => {
                        let project_root_path: &str = self.input.config_project_root.as_ref().unwrap();
                        let project_root_path = &project_root_path.replace("  ", " ");
                        let project_root: Result<fs::ReadDir, _> = fs::read_dir(project_root_path);
                        match project_root{
                            Ok(root) => info::Info::list_clients(root),
                            Err(message) => panic!("{}",message)
                        }
                    },
                    _ => ()
                }
            }, // handle this later
            (None, None, None) => {
                match &self.input.arguments {
                    Some(arg) => {
                        let arg_type: ArgumentType = Program::argument_type(&arg.get(0).unwrap());
                        match arg_type {
                            ArgumentType::ProjectCode(_project_code) => (),
                            ArgumentType::ClientCode(_client_code) => (),
                            ArgumentType::ClientName(_client_name) => (),
                            ArgumentType::Current => self.print_current()?,
                            ArgumentType::Active => self.print_active()?,
                            ArgumentType::New(_input_kind) => (),
                            ArgumentType::Add => (),
                        }
                        println!("programming for different arguments needed")
                    }
                    None => {
                        // open both the main project and main production folders
                        Program::open_path_in_window(&project_dir)?;
                        Program::open_path_in_window(&production_dir)?;
                    }

                }
            },
            _ => {
                // get paths as string
                let proj_path: std::path::PathBuf = project_dir.path();
                let proj_path: &str = proj_path.to_str().unwrap();
                let prod_path: std::path::PathBuf = production_dir.path();
                let prod_path: &str = prod_path.to_str().unwrap();

                // get and open folders that correspond to flags
                if flags_proj.is_some(){
                    flags_proj.unwrap().iter().for_each(| f |{
                        let proj_content: fs::ReadDir = fs::read_dir(&proj_path).unwrap();
                        match f {
                            'c' => {
                                let mut dir: Result<fs::DirEntry, String> = Program::find_dir_in_dir_matching_anywhere_in_name(proj_content, "Client");
                                match dir {
                                    Ok(dir) => Program::open_path_in_window(&dir).unwrap(),
                                    Err(_) => {
                                        // this solution is necessary to make pnav compatible with
                                        // current project management at maerschalk
                                        let proj_content: fs::ReadDir = fs::read_dir(&proj_path).unwrap();  
                                        dir = Program::find_dir_in_dir_matching_anywhere_in_name(proj_content, "Klant");
                                        match dir {
                                            Ok(dir) => Program::open_path_in_window(&dir).unwrap(),
                                            Err(message) => panic!("{}", message)
                                        }

                                    }
                                }
                            },
                            's' => {
                                let mut dir: Result<fs::DirEntry, String> = Program::find_dir_in_dir_matching_anywhere_in_name(proj_content, "Supplier");
                                match dir {
                                    Ok(dir) => Program::open_path_in_window(&dir).unwrap(),
                                    Err(_) => {
                                        // this solution is necessary to make pnav compatible with
                                        // current project management at maerschalk
                                        let proj_content: fs::ReadDir = fs::read_dir(&proj_path).unwrap();  
                                        dir = Program::find_dir_in_dir_matching_anywhere_in_name(proj_content, "Leverancier");
                                        match dir {
                                            Ok(dir) => Program::open_path_in_window(&dir).unwrap(),
                                            Err(message) => panic!("{}", message)
                                        }

                                    }
                                }
                            },
                            'm' => {
                                let company_name = "Maerschalk";
                                let dir: fs::DirEntry = Program::find_dir_in_dir_matching_anywhere_in_name(proj_content, company_name).unwrap();
                                Program::open_path_in_window(&dir).unwrap();
                            },
                            _ => ()
                        }
                    })
                }

                if flags_prod.is_some(){
                    flags_prod.unwrap().iter().for_each(| f |{
                        let prod_content: fs::ReadDir = fs::read_dir(&prod_path).unwrap();
                        match f {
                            'a' => {
                                let dir: fs::DirEntry = Program::find_dir_in_dir_matching_anywhere_in_name(prod_content, "Asset").unwrap();
                                Program::open_path_in_window(&dir).unwrap();
                            },
                            'p' => {
                                let dir: fs::DirEntry = Program::find_dir_in_dir_matching_anywhere_in_name(prod_content, "Project").unwrap();
                                Program::open_path_in_window(&dir).unwrap();
                            },
                            'd' => {
                                let dir: fs::DirEntry = Program::find_dir_in_dir_matching_anywhere_in_name(prod_content, "Deliver").unwrap();
                                Program::open_path_in_window(&dir).unwrap();
                            },
                            _ => ()
                        }
                    })
                }
            }
        }

        Ok(())
    }

}

impl Program {
    fn get_projects_root_dir(&self) -> Result<fs::ReadDir, String>{
        // get project root as text
        let project_root: &str = match &self.input.config_project_root{
            Some(path) => path,
            None => panic!("unable to find project root folder")
        };

        // remove redundand spaces
        let project_root = project_root.replace("  ", " ");
        match fs::read_dir(project_root){
            Ok(dir) => Ok(dir),
            Err(_message) => Err(String::from("projects root directory not found"))
        }
    }
    fn get_project_dir(&self) -> Result<fs::DirEntry, String>{
        // get project code
        let project_code = self.input.config_active_project.as_ref().expect("cannot find a project code");

        // get client folders from project root
        let client_folders: fs::ReadDir = self.get_projects_root_dir()?;

        let client_root: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(client_folders, &project_code[0..3]);
        let project_root: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(fs::read_dir(client_root.path()).expect("unable to read client root directory"), &project_code[0..6]);

        Ok(project_root)
    }
    fn get_production_dir(&self)->Option<fs::DirEntry>{
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
                    first_six_chars = &name_of_dir[0..6];
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

        prod_dir
    }
    fn print_current(&self) -> Result<(), String>{
        // get shorthands
        let projects_root_dir: fs::ReadDir = self.get_projects_root_dir()?;
        let project_code: &str = &self.input.config_active_project.as_ref().expect("no active project code found");

        // get directory from current project code
        let client_dir: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(projects_root_dir, &project_code[0..3]);
        let project_dir: fs::DirEntry = Program::find_dir_in_dir_matching_from_start_of_name(fs::read_dir(client_dir.path()).unwrap(), &project_code);
        let project_dir_name: String = project_dir.file_name().into_string().unwrap();

        // print
        let mut client_name: Vec<&str> = project_dir_name.split(" - ").collect();
        let client_name: &str = client_name.remove(2);
        let mut project_name: Vec<&str> = project_dir_name.split(" - ").collect();
        let project_name: &str = project_name.remove(3);
        println!("\n{}\n--------------","info:");
        println!("{}:\t\t{}",Green.normal().paint("project code"),Yellow.normal().paint(project_code));
        println!("{}:\t\t\t{}",Green.normal().paint("client"),Yellow.normal().paint(client_name));
        println!("{}:\t\t{}",Green.normal().paint("project name"),Yellow.normal().paint(project_name));
        println!("{}\n","--------------");
        Ok(())
    }
    fn print_active(&self) -> Result<(), String> {
        // define number of seconds in an active period
        // every client and project that has been modified in this time, wil be printed
        let active_time_span: std::time::Duration = std::time::Duration::from_secs(3888000); // approximately 3 months

        // get projects
        let projects_root: fs::ReadDir = self.get_projects_root_dir()?;

        // get list of active client folders
        let mut active_clients: Vec<fs::DirEntry> = projects_root
            .map(| dir | {
                let dir: fs::DirEntry = dir.unwrap();
                dir
            }).filter(| dir |{
                let dir_elapsed_time: std::time::Duration = dir.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .elapsed()
                    .unwrap();

                dir_elapsed_time < active_time_span
            }).collect();

        // get list of active project folders
        let mut active_projects: Vec<fs::DirEntry> = vec![];
        active_clients.iter().for_each(| c_dir |{
            let p_dirs: Result<fs::ReadDir, _> = fs::read_dir(c_dir.path());
            match p_dirs {
                Ok(p_dirs) => {
                    let mut p_dirs: Vec<fs::DirEntry> = p_dirs
                        .map(|p_dir|{
                            p_dir.unwrap()
                        }).filter(|p_dir|{
                            let dir_elapsed_time: std::time::Duration = p_dir.metadata()
                                .unwrap()
                                .modified()
                                .unwrap()
                                .elapsed()
                                .unwrap();
                            dir_elapsed_time < active_time_span && p_dir.metadata().unwrap().is_dir()
                        }).collect();

                    // store p_dirs in active_projects vector
                    for _ in 0..p_dirs.len(){
                       active_projects.push(p_dirs.remove(0)) 
                    };
                }
                Err(_) => ()
            }
        });

        // sort directories
        Program::sort_dir_entries(&mut active_projects);
        Program::sort_dir_entries(&mut active_clients);

        // print data for user
        println!("\n---------------------------------------------------------");
        println!("{}",Yellow.paint("these projects have been active in the past three months:"));
        println!("---------------------------------------------------------\n");
        active_clients.iter().for_each(|c_dir|{
            if c_dir.file_name().to_str().expect("should retreive str from file name").chars().next().expect("should return the first character of the string").is_numeric(){
                // print client name
                let c_name: String = c_dir.file_name().into_string().expect("should convert to string");
                let c_name: &str = c_name.split(" - ").last().expect("should return client name");
                println!("{}",Yellow.paint(c_name));

                // get client code
                let client_code: std::ffi::OsString = c_dir.file_name();
                let client_code: &str = client_code
                    .to_str().expect("should return filename as string")
                    .split(" ")
                    .next().expect("should return client code from folder name");

                // loop through project dirs and check which project belongs to this client
                active_projects.iter().for_each(|p_dir|{
                    let p_folder_name: String = p_dir.file_name().into_string().expect("should return name of project folder as string");
                    let mut p_folder_name_chars: std::str::Chars = p_folder_name.chars();

                    // retreive first three chars from project folder name
                    let mut first_three_chars: String = String::new(); 
                    for _ in 0..3 {
                        first_three_chars.push_str(&format!("{}",p_folder_name_chars.next().expect("this filename does not have enough chars for this method")));
                    }

                    if client_code == &first_three_chars {
                        let mut name_parts: std::str::Split<&str> = p_folder_name.split(" - "); 
                        let proj_code: &str = name_parts.next().unwrap();
                        let proj_name: &str = name_parts.last().unwrap();
                        println!("{} - {}", Green.paint(proj_code), Purple.paint(proj_name));
                    }
                });
                println!("\n");
            }

        });
        println!("---------------------------------------------------------");

        Ok(())
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
    fn find_dir_in_dir_matching_anywhere_in_name(read_dir: fs::ReadDir, query: &str) -> Result<fs::DirEntry, String>{
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
                if c.contains(query) {
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

        match matching_dirs.len(){
            0 => Err(String::from("no directory found")),
            _ => Ok(matching_dirs.remove(0))
        }
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
    fn argument_type(arg: &str)->ArgumentType{
        if arg.len() == 6 && arg.chars().filter(| a | a.is_numeric()).count() == 6{
            ArgumentType::ProjectCode(String::from(arg))
        } else if arg.len() == 3 && arg.chars().filter(| a | a.is_numeric()).count() == 3 {
            ArgumentType::ClientCode(String::from(arg))
        } else if arg.to_lowercase().contains("current") {
            ArgumentType::Current
        } else if arg.to_lowercase().contains("active") {
            ArgumentType::Active
        } else if arg.to_lowercase().contains("add") {
            ArgumentType::Add
        } else if arg.to_lowercase().contains("new") {
            ArgumentType::New(ArgumentNewType::Nothing)
        } else {
            ArgumentType::ClientName(String::from(arg))
        }
    }
    fn sort_dir_entries(dirs: &mut Vec<fs::DirEntry>) {
        dirs.sort_unstable_by(|a, b|{
            let a_first_six: String = a.file_name().into_string().expect("file name should convert to string");
            let a_first_six: &str = a_first_six.split(" ").next().expect("should return project code in dir name"); 
            let b_first_six: String = b.file_name().into_string().expect("file name should convert to string");
            let b_first_six: &str = b_first_six.split(" ").next().expect("should return project code in dir name"); 
            a_first_six.partial_cmp(&b_first_six).expect("should have sorted entry")
        });
    }  

}
