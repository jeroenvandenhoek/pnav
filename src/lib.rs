use std::fs;
use std::error::Error;
mod process_arguments;
mod manipulate_pnavrc;
mod contextual_info;
pub mod navigate_folders;
mod create_project_directories;

/// runs the pnav program
pub fn run(args: Vec<String>) {
    manipulate_pnavrc::read_pnavrc();
    let client_input_folder: Result<fs::DirEntry, _> = navigate_folders::get_project_input_folder("150", "/root/pnav_project_root_for_testing");
    let project_input_folder: Result<fs::DirEntry, _> = navigate_folders::get_project_input_folder("150001", "/root/pnav_project_root_for_testing");

    println!("{:?}",client_input_folder);
    println!("{:?}",project_input_folder);
}
