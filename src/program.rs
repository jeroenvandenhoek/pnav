use super::input;
use super::info;
use std::error::Error;
use std::fs;
use std::env;
use run_script;

enum ArgumentType {
    ProjectCode(String),
    ClientCode(String),
    ClientName(String),
    Current,
    New,
    Add,
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
            (Some(gen), _, _) => {
                if gen.contains(&'i'){
                    match &self.input.arguments{
                        Some(args) => {
                            match args.get(0) {
                                Some(arg) => {
                                    match Program::argument_type(arg){
                                        ArgumentType::ProjectCode(project_code) => {
                                            println!("\nhere you'll see info about project: {}\n",project_code);
                                        },
                                        ArgumentType::ClientCode(client_code) =>{
                                            println!("\nhere you'll see info about client: {}\n",client_code);
                                        },
                                        ArgumentType::ClientName(client_name) => {
                                            ()
                                        },
                                        ArgumentType::Current => {
                                            Program::print_current(self.input.config_active_project.as_ref().expect("no project code present"))?;
                                        },
                                        ArgumentType::New => (),
                                        ArgumentType::Add => (),
                                        _ => ()
                                    };
                                    ()
                                },
                                None => ()

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
                        }
                    }
                }
            }, // handle this later
            (None, None, None) => {
                match &self.input.arguments {
                    Some(arg) => {
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
    fn print_current(project_code: &str) -> Result<(), String>{
        println!("This is the current project:\ncode:\t\t{}\nclient:\t\t{},\nproject name:\t{}","123456","troelala", "oelala");
        Ok(())
    }
    fn argument_type(arg: &str)->ArgumentType{
        if arg.len() == 6 && arg.chars().filter(| a | a.is_numeric()).count() == 6{
            ArgumentType::ProjectCode(String::from(arg))
        } else if arg.len() == 3 && arg.chars().filter(| a | a.is_numeric()).count() == 3 {
            ArgumentType::ClientCode(String::from(arg))
        } else if arg.to_lowercase().contains("current") {
            ArgumentType::Current
        } else if arg.to_lowercase().contains("add") {
            ArgumentType::Add
        } else if arg.to_lowercase().contains("new") {
            ArgumentType::New
        } else {
            ArgumentType::ClientName(String::from(arg))
        }
    }

}
