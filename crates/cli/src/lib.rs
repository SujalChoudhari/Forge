use add::add_recipe_to_forge_from_user;
use help::print_help_message;

pub mod add;
pub mod help;

/// handles cli commands if any
/// #### Params
/// - command - [Vec] String: Nameless commands.
/// #### Returns
/// - `true` when command is handled
/// - `false` when no such command is found
pub fn handle_cli_command(command: &Vec<String>) -> bool {
    let root_command = command.get(1).unwrap_or(&&"help".to_string()).to_owned();
    match root_command.as_str() {
        "add" => {
            add_recipe_to_forge_from_user();
            return true;
        }
        "help" => {
            print_help_message();
            return true;
        }
        _ => {
            return false;
        }
    };
}
