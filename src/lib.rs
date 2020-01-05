mod process_arguments;
mod manipulate_pnavrc;
mod contextual_info;
mod navigate_folders;
mod create_project_directories;

pub fn run(args: Vec<String>) {
    manipulate_pnavrc::manip_pnavrc();
    navigate_folders::get_project_folder("002003");
}
