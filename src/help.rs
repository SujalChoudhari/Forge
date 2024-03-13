use crate::{constants::*, logging::IS_VERBOSE};
use colored::Colorize;
use std::collections::HashMap;

pub fn print_help_message() {
    let mut commands = HashMap::new();
    commands.insert(
        "-h, --help",
        "Print help information. Use -h -V for verbose output.",
    );
    commands.insert("-v, --version", "Print version information");
    commands.insert("-V, --verbose", "Print verbose output");

    let mut internal_vars = HashMap::new();
    internal_vars.insert(
        OS_VARIABLE_NAME,
        "The current OS (possible values: win, linux, mac)",
    );
    internal_vars.insert(
        FILE_PATH_VARIABLE_NAME,
        "The complete path to the current file",
    );
    internal_vars.insert(FILE_NAME_VARIABLE_NAME, "The name of the current file");
    internal_vars.insert(
        FILE_NAME_EXT_VARIABLE_NAME,
        "The name of the current file with extension",
    );
    internal_vars.insert(FILE_EXT_VARIABLE_NAME, "The extension of the current file");
    internal_vars.insert(FILE_DIR_VARIABLE_NAME, "The directory of the current file");

    print_help_message_raw(&commands, &internal_vars);
}

fn print_help_message_raw(commands: &HashMap<&str, &str>, internal_vars: &HashMap<&str, &str>) {
    println!(
        "{}: {}",
        APP_NAME.on_cyan().truecolor(0, 0, 0),
        APP_VERSION.magenta().italic(),
    );

    println!(
        "    {}: {}:",
        "USAGE".bold().green(),
        "(Options)".truecolor(0, 100, 0)
    );

    for (option, description) in commands.iter() {
        println!(
            "\t-\t{}\t{}",
            option.yellow(),
            description.truecolor(150, 150, 150)
        );
    }

    println!("\n    {}:", "COMMANDS".bold().green());
    println!(
        "\t\t{} <recipe>  <vars...> {}",
        APP_FILENAME.cyan(),
        "Run a command with given flags and variables. Variables are set using --<key>=<value>"
            .truecolor(150, 150, 150)
    );

    println!(
        "\n    {}:",
        "INTERNAL VARIABLES (available as 'variables' command)"
            .bold()
            .green()
    );
    for (var_name, description) in internal_vars.iter() {
        println!(
            "\t-\t{}   \t{}",
            var_name.yellow(),
            description.truecolor(150, 150, 150)
        );
    }

    if unsafe { IS_VERBOSE } {
        println!(
            "\n    {}:{}",
            "VARIABLE REPLACEMENT".bold().green(),
            "\n\t-\tVariables can be used in commands and file paths enclosed in curly braces, e.g. {os} or {fileDir}\n\t-\tIndexed variables can be used with a number inside the curly braces, e.g. {$fileName}".truecolor(150, 150, 150)
        );
        println!(
            "\n    {}:{}",
            "DETECTION PATTERN".bold().green(),
            &format!("Default detection pattern is '{}'", DEFAULT_DETECT_PATTERN)
                .truecolor(150, 150, 150)
        );
        println!(
            "    {}:{}",
            "FILE PATH".bold().green(),
            &format!("The default file path is '{}'", APP_FILENAME_DEFAULT_PATH)
                .truecolor(150, 150, 150)
        );
        println!(
            "    {}:{}",
            "DEFAULT RECIPE".bold().green(),
            &format!("The default recipe is '{}'", DEFAULT_RECIPE).truecolor(150, 150, 150)
        );
        println!(
            "    {}:{}",
            "ROOT DIRECTORY".bold().green(),
            &format!("The default directory is '{}'", DEFALUT_DIR).truecolor(150, 150, 150)
        );
    }

    println!(
        "    {}:\t {} {}",
        "SEE ALSO".bold().underline().blue(),
        "Forge recipe documentation for more information on recipes and variables"
            .truecolor(150, 150, 150),
        "https://github.com/SujalChoudhari/Forge".blue().underline()
    );
}
