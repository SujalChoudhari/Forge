use constants::{ALWAYS_KEY, APP_SUBTITLE, DETECT_KEY, OS_KEY, RUN_KEY, VARIABLES_KEY};
use logger::Logger;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec;

pub fn add_recipe_to_forge(
    name: &str,
    os: Vec<String>,
    detect: Vec<String>,
    always: bool,
    vars: HashMap<String, String>,
    run: Vec<String>,
) {
    let forge_file_path = "./ForgeFile";

    // Create a new YAML string for the recipe
    let yaml_string = format!(
        "\n# Generated using forge add. {}\n{}:\n  {}: {:?}\n  {}: {:?}\n  {}: {}\n  {}: {:?}\n  {}: {:?}\n",
        APP_SUBTITLE,name, OS_KEY,os, DETECT_KEY,detect,ALWAYS_KEY ,always,VARIABLES_KEY, vars,RUN_KEY, run
    );

    // Open ForgeFile or create if it doesn't exist
    let mut forge_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(forge_file_path)
        .expect("Failed to open ForgeFile");

    // Write the YAML string to ForgeFile
    writeln!(forge_file, "{}", yaml_string).expect("Failed to write recipe to ForgeFile");
}

pub fn add_recipe_to_forge_from_user() {
    // Ask user for recipe name
    let name = Logger::input_autocomplete(
        "Enter recipe name: ",
        vec!["forge", "build", "push", "add", "test"],
    );

    // Ask user for list of operating systems
    let os: Vec<String> = Logger::input_multiselect(
        "Select operating systems to run this recipe on: ",
        vec!["all", "win", "mac", "linux"],
    );

    // Ask user for list of detection parameters
    let detect = Logger::input_multiselect(
        "Enter select detection parameters: ",
        vec!["*", "src/", "*.rs", "*.js", "test/", "dist/", "*.py"],
    );

    // Ask user for 'always' flag
    let always_input = Logger::input_choice(
        "Should the recipe always run?: ",
        vec!["true", "false"],
    );
    let always = always_input.trim().parse().unwrap_or(true);

    // Ask user for variables
    let vars_input =
        Logger::input_default("Enter variables (key-value pairs separated by comma): ", "");
    let vars: HashMap<String, String> = if !vars_input.trim().is_empty() {
        vars_input
            .split(',')
            .map(|pair| {
                let mut iter = pair.split('=');
                let key = iter.next().unwrap().trim().to_owned();
                let value = iter.next().unwrap_or("").trim().to_owned();
                (key, value)
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Ask user for list of commands to run
    let run_input = Logger::input_default(
        "Enter list of commands to run (comma-separated): ",
        "echo forge",
    );
    let run: Vec<String> = run_input.split(',').map(|s| s.trim().to_owned()).collect();

    // Call add_recipe_to_forge function with user-provided inputs
    Logger::info(&["Recipe ", &name, " is added."].concat());
    add_recipe_to_forge(&name, os, detect, always, vars, run);
}
