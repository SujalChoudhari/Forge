use add::add_recipe_to_forge_from_user;
use help::print_help_message;
use menu::run_menu_handler;
use template::run_templating_engine;

pub mod add;
pub mod help;
pub mod menu;
pub mod template;

/// handles cli commands if any
/// #### Params
/// - command - [Vec] String: Nameless commands.
/// #### Returns
/// - `true` when command is handled
/// - `false` when no such command is found
pub fn handle_cli_command(command: &mut Vec<String>) -> bool {
    let root_command = command.get(1).unwrap_or(&&"menu".to_string()).to_owned();
    match root_command.as_str() {
        "add" => {
            add_recipe_to_forge_from_user();
            return true;
        }
        "help" => {
            print_help_message();
            return true;
        }
        "template" | "from" => {
            run_templating_engine();
            return true;
        }
        "menu" => {
            let command_to_execute = run_menu_handler();
            if command.len() >= 2 {
                command[1] = command_to_execute;
            } else {
                command.push(command_to_execute);
            }

            return false;
        }
        _ => {
            return false;
        }
    };
}
