use colored::Colorize;
use constants::*;
use logger::Logger;
use std::collections::HashMap;

pub fn print_help_message() {
    let mut flags: HashMap<(&str, &str), &str> = HashMap::new();
    flags.insert(
        HELP_FLAG,
        "Print help information. Use -h -V for verbose help menu.",
    );
    flags.insert(VERSION_FLAG, "Print version information");
    flags.insert(VERBOSE_FLAG, "Print verbose output");
    flags.insert(
        FORCE_EXECUTE_FLAG,
        "Allows execution of all commands despite encountered errors.",
    );

    let mut commands: HashMap<&str, &str> = HashMap::new();
    commands.insert(
        DEFAULT_RECIPE,
        "Run a command with given flags and variables. 
    \t\t\t\tVariables are set using --<key>=<value>",
    );

    let add_command_description = format!(
        "Adds a recipe to the current {} 
    \t\t\t\tQuestions will be asked for initializing",
        APP_FILENAME
    );
    commands.insert("forge add", &add_command_description);
    commands.insert("forge help", "Shows this menu.");
    commands.insert("forge from", "Create a new project from a template. Alias to `template`");
    commands.insert("forge menu", "Starts a choice menu to run any recipe in forgefile.");

    let mut internal_vars = HashMap::new();
    internal_vars.insert(
        OS_VARIABLE_NAME,
        "\tThe current OS (possible values: win, linux, mac)",
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

    print_help_message_raw(&flags, &commands, &internal_vars);
}

fn print_help_message_raw(
    flags: &HashMap<(&str, &str), &str>,
    commands: &HashMap<&str, &str>,
    internal_vars: &HashMap<&str, &str>,
) {
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

    for (option, description) in flags.iter() {
        println!(
            "\t-\t-{},--{}\t{}",
            option.0.yellow(),
            option.1.yellow(),
            description.truecolor(150, 150, 150)
        );
    }
    // Print COMMANDS section
    println!("\n    {}:", "COMMANDS".bold().green());
    // println!(
    //     "\t\t{} <recipe>  <vars...> {}",
    //     DEFAULT_RECIPE.cyan(),
    //     "Run a command with given flags and variables.
    //     \t\t\t\tVariables are set using --<key>=<value>"
    //         .truecolor(150, 150, 150)
    // );
    for (var_name, description) in commands.iter() {
        println!(
            "\t-\t{}   \t{}",
            var_name.yellow(),
            description.truecolor(150, 150, 150)
        );
    }

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

    if Logger::get_is_verbose() {
        // Print VARIABLE REPLACEMENT, DETECTION PATTERN, FILE PATH, DEFAULT RECIPE, DEFAULT DIRECTORY, and SEE ALSO sections
        println!(
            "\n    {}:{}",
            "VARIABLE REPLACEMENT".bold().green(),
            "\n\t-\tVariables can be used in commands and file paths 
        \tenclosed in curly braces, e.g. {os} or {fileDir}
        -\tIndexed variables can be used with a number inside 
        \tthe curly braces, e.g. {$fileName}"
                .truecolor(150, 150, 150)
        );
        println!("");
        println!(
            "    {}:\t {} {}",
            "DETECTION PATTERN".bold().green(),
            DEFAULT_DETECT_PATTERN.yellow(),
            "Default when recipe doesn't contain detect tag.".truecolor(150, 150, 150)
        );
        println!(
            "    {}:\t\t {} {}",
            "FILE PATH".bold().green(),
            APP_FILENAME_DEFAULT_PATH.yellow(),
            "Allowed filepath for execution. Other files will be ignored.".truecolor(150, 150, 150)
        );
        println!(
            "    {}:\t {} {}",
            "DEFAULT RECIPE".bold().green(),
            DEFAULT_RECIPE.yellow(),
            "Used when no recipe is specified. Allows default execution".truecolor(150, 150, 150)
        );
        println!(
            "    {}:\t {} {}",
            "ROOT DIRECTORY".bold().green(),
            DEFALUT_DIR.yellow(),
            "Forge will start in the current directory of forge file.".truecolor(150, 150, 150)
        );
    }
    println!(
        "    {}:\t\t {}\n\t\t\t [ctrl + click]{}",
        "SEE ALSO".bold().underline().blue(),
        "Forge recipe documentation for more information on recipes and variables"
            .truecolor(150, 150, 150),
        "https://github.com/SujalChoudhari/Forge".blue().underline()
    );
}
