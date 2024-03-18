use std::process::Command;
use std::string::String;

use logger::Logger;

/// Executes the given command on the current operating system.
/// This function is platform-independent.
///
/// #### Parameters
/// - `command`: &[str] - The command to execute. This can be a shell command, system command, etc.
///
/// #### Complexity
/// - O(m)
///   - Where `m` is the complexity of the provided command, typically influenced by its length.
///
/// #### Returns
/// - `Result<(String, String), String>`: A tuple containing the standard output and standard error of the command execution.
///   - `Ok((stdout, stderr))`: If the command executed successfully, `stdout` contains the standard output, and `stderr` contains the standard error (if any).
///   - `Err(error_message)`: If an error occurred during command execution, `error_message` provides details about the failure.

pub fn execute(command: &str) -> Result<String, String> {
    let mut is_error = false;
    let command_result: Result<std::process::Output, std::io::Error> =
        if cfg!(target_os = "windows") {
            Command::new("powershell")
                .arg("-Command")
                .arg(command)
                .output()
        } else {
            Command::new("sh").arg("-c").arg(command).output()
        };

    let raw_output;
    match command_result {
        Result::Ok(out) => {
            if out.status.code().unwrap_or(0) != 0 {
                is_error = true;
            }
            if out.stdout.len() > 0 {
                raw_output = out.stdout;
            } else {
                raw_output = out.stderr;
            }
        }
        Result::Err(_) => {
            Logger::error(&["Cannot run \"", command, "\" on current OS"].concat());
            raw_output = vec![];
        }
    };

    let viewable_output: String;
    match String::from_utf8(raw_output) {
        Ok(ok) => viewable_output = ok,
        Err(_) => {
            Logger::warn("Cannot convert output to viewable string.");
            viewable_output = String::new();
        }
    };

    if is_error {
        Logger::intermidiate_error(command, &viewable_output);
        return Result::Err(viewable_output);
    }
    Logger::intermidiate_info(command, &viewable_output);
    Result::Ok(viewable_output)
}
