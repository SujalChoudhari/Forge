use constants::APP_SUBTITLE;
use logger::{info, input};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};

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
        "\n# Generated using forge add. {}\n{}:\n  os: {:?}\n  detect: {:?}\n  always: {}\n  vars: {:?}\n  run: {:?}\n",
        APP_SUBTITLE,name, os, detect, always, vars, run
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
    let name = input("Enter recipe name: ", "forge");

    // Ask user for list of operating systems
    let os_input = input("Enter list of operating systems (comma-separated): ", "all");
    let os: Vec<String> = os_input.split(',').map(|s| s.trim().to_owned()).collect();

    // Ask user for list of detection parameters
    let detect_input = input(
        "Enter list of detection parameters (comma-separated): ",
        "*",
    );
    let detect: Vec<String> = detect_input
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect();

    // Ask user for 'always' flag
    let always_input = input("Should the recipe always run? (true/false): ", "true");
    let always = always_input.trim().parse().unwrap_or(true);

    // Ask user for variables
    let vars_input = input("Enter variables (key-value pairs separated by comma): ", "");
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
    let run_input = input(
        "Enter list of commands to run (comma-separated): ",
        "echo forge",
    );
    let run: Vec<String> = run_input.split(',').map(|s| s.trim().to_owned()).collect();

    // Call add_recipe_to_forge function with user-provided inputs
    info(&["Recipe ", &name, " is added."].concat());
    add_recipe_to_forge(&name, os, detect, always, vars, run);
}
