use std::fs::OpenOptions;
use std::io::Write;

use constants::{recipie_templates::recipe_templates, APP_FILENAME_DEFAULT_PATH};
use logger::Logger;

fn append_template_to_forge(content: &str) {
    // Open ForgeFile or create if it doesn't exist
    let mut forge_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(APP_FILENAME_DEFAULT_PATH)
        .expect("Failed to open ForgeFile");

    // Write the YAML string to ForgeFile
    writeln!(forge_file, "{}", content).expect("Failed to write recipe to ForgeFile");
}

fn get_selected_templates() -> Vec<String> {
    Logger::input_multiselect(
        &[
            "Select all the templates to import: (",
            recipe_templates().len().to_string().as_str(),
            " total)",
        ]
        .concat(),
        recipe_templates().keys().into_iter().map(|f| *f).collect(),
    )
}

pub fn run_templating_engine() {
    // Assuming get_selected_templates() returns an iterator over selected template names
    // And recipe_templates() returns a reference to the static hashmap
    for selected_template in get_selected_templates() {
        // Attempt to retrieve the template associated with the selected key
        if let Some(template) = recipe_templates().get(&selected_template.as_str()) {
            append_template_to_forge(template);
        } else {
            Logger::error(
                &[
                    "Template '",
                    selected_template.as_str(),
                    "' not found in recipe templates.",
                ]
                .concat(),
            );
        }
    }

    Logger::info("Templates added.");
}
