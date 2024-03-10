use argparser::load_command_line_arguents;
use filehandler::get_all_files_in_directory;
use variables::Variables;

use crate::{
    commander::execute,
    filehandler::{
        did_other_files_changed, get_files_in_directory_with_criteria, get_last_modified_of_files, update_last_modified_of_files,
    },
    parser::load_forge,
};

pub mod argparser;
pub mod commander;
pub mod filehandler;
pub mod interpreter;
pub mod logging;
pub mod parser;
pub mod variables;

fn main() {
    let out = get_files_in_directory_with_criteria("./", &["src/".to_string()]);
    let last_mod = get_last_modified_of_files(&out);
    let has_changed = did_other_files_changed(
        get_files_in_directory_with_criteria("./", &["*.forge".to_string()])
            .get(0)
            .unwrap()
            .to_owned(),
        out,
    );

    update_last_modified_of_files(get_files_in_directory_with_criteria("./", &["*.forge".to_string()]));
    println!("{:?}", has_changed);
}
