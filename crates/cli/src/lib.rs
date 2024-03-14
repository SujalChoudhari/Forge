use add::add_recipe_to_forge_from_user;

pub mod help;
pub mod add;


pub fn handle_cli_command(command:&str) -> bool {

    if command == "add" {
        add_recipe_to_forge_from_user();
    }

    true
}