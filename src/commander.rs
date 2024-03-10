use std::process::Command;
use std::string::String;

use crate::logging::{error, intermidiate_error, intermidiate_info, warn};

pub fn execute(command: &str) -> Result<String, String> {
    let mut is_error = false;
    let command_result = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(command).output()
    } else {
        Command::new("sh").arg("-c").arg(command).output()
    };

    let raw_output;
    match command_result {
        Result::Ok(out) => {
            if out.stdout.len() > 0 {
                raw_output = out.stdout;
            } else {
                raw_output = out.stderr;
                is_error = true;
            }
        }
        Result::Err(_) => {
            error(&["Cannot run \"", command, "\" on current OS"].concat());
            raw_output = vec![];
        }
    };

    let viewable_output: String;
    match String::from_utf8(raw_output) {
        Ok(ok) => viewable_output = ok,
        Err(_) => {
            warn("Cannot convert output to viewable string.");
            viewable_output = String::new();
        }
    };

    if is_error {
        intermidiate_error(command, &viewable_output);
        return Result::Err(viewable_output);
    }
    intermidiate_info(command, &viewable_output);
    Result::Ok(viewable_output)
}
