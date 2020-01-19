use std::fs;
use std::error::Error;

mod input;
mod program;

/// runs the pnav program
pub fn run() {
    let input = input::Input::get();
    let program = program::Program::run(input);


}
