use regex::Regex;
use std::{collections::HashMap, vec};

use constants::{
    FILE_DIR_VARIABLE_NAME, FILE_EXT_VARIABLE_NAME, FILE_NAME_EXT_VARIABLE_NAME,
    FILE_NAME_VARIABLE_NAME, FILE_PATH_VARIABLE_NAME, VARIABLE_REPLACE_TEMPLATE,
    VARIABLE_REPLACE_WITH_INDEX_TEMPLATE,
};
use logger::Logger;

#[derive(Debug)]
pub struct Variables {
    memory: HashMap<String, Vec<String>>,
}

impl Variables {
    pub fn new() -> Self {
        Variables {
            memory: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: &String) {
        // Insert the key-value pair into the HashMap
        self.memory
            .entry(key)
            .or_insert_with(Vec::new)
            .push(value.to_owned());
    }

    pub fn add_from_hash(&mut self, map: &HashMap<String, Vec<String>>) {
        for (key, values) in map {
            for value in values {
                self.add(key.clone(), &value);
            }
        }
    }

    pub fn add_vec(&mut self, key: String, value: Vec<String>) {
        self.memory.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&Vec<String>> {
        // Retrieve the value (Vec<String>) for the given key
        self.memory.get(key)
    }

    pub fn exists(&self, key: &String) -> bool {
        // Check if the key exists in the HashMap
        self.memory.contains_key(key)
    }

    fn replace_key_with_index(
        &self,
        input_string: String,
        key_to_replace: &str,
        result: &mut Vec<String>,
    ) {
        // Iterate over each value of the key
        if let Some(values) = self.memory.get(key_to_replace) {
            for value in values {
                // Replace the key_to_replace with the actual value
                let index_of_value = match values.binary_search(value) {
                    Result::Ok(ok) => ok,
                    Result::Err(err) => err,
                };
                let replaced_string = input_string
                    .replace(
                        &[
                            VARIABLE_REPLACE_WITH_INDEX_TEMPLATE.0,
                            key_to_replace,
                            VARIABLE_REPLACE_WITH_INDEX_TEMPLATE.1,
                        ]
                        .concat(),
                        &index_of_value.to_string(),
                    )
                    .replace(
                        &[
                            VARIABLE_REPLACE_TEMPLATE.0,
                            key_to_replace,
                            VARIABLE_REPLACE_TEMPLATE.1,
                        ]
                        .concat(),
                        value,
                    );
                // Add the replaced string to the result vector
                result.push(replaced_string);
            }
        }
    }

    fn replace_key_with_value(
        &self,
        input_string: String,
        key_to_replace: &str,
        result: &mut Vec<String>,
    ) {
        // Iterate over each value of the key
        if let Some(values) = self.memory.get(key_to_replace) {
            for value in values {
                // Replace the key_to_replace with the actual value
                let replaced_string = input_string.replace(
                    &[
                        VARIABLE_REPLACE_TEMPLATE.0,
                        key_to_replace,
                        VARIABLE_REPLACE_TEMPLATE.1,
                    ]
                    .concat(),
                    value,
                );

                // Add the replaced string to the result vector
                result.push(replaced_string);
            }
        }
    }

    fn replace_key(&self, input_strings: Vec<String>, key_to_replace: &str) -> Vec<String> {
        let mut result = Vec::new();

        for input_string in input_strings {
            // Check if the key_to_replace is present in the input string
            if input_string
                .contains(&[VARIABLE_REPLACE_WITH_INDEX_TEMPLATE.0, key_to_replace].concat())
            {
                self.replace_key_with_index(input_string, key_to_replace, &mut result);
            } else if input_string.contains(key_to_replace) {
                self.replace_key_with_value(input_string, key_to_replace, &mut result);
            } else {
                result.push(input_string);
            }
        }

        result
    }

    pub fn format_templates(&mut self, input_strings: Vec<String>) -> Vec<String> {
        // Create a BTreeMap to store keys sorted by the length of their values
        let mut special_vars_formatted_strings: Vec<String> = vec![];

        for string in &input_strings {
            let inputted_string: &String = &self.replace_placeholders(string);
            special_vars_formatted_strings.extend(self.replace_special_vars(&inputted_string));
        }
        // Iterate over sorted keys and apply replacements
        for key in self.memory.keys() {
            special_vars_formatted_strings = self.replace_key(special_vars_formatted_strings, key);
        }

        special_vars_formatted_strings
    }

    fn replace_special_vars(&self, input_string: &String) -> Vec<String> {
        let mut modified_string: Vec<String> = vec![];
        if !input_string.contains(FILE_NAME_VARIABLE_NAME)
            && !input_string.contains(FILE_PATH_VARIABLE_NAME)
            && !input_string.contains(FILE_DIR_VARIABLE_NAME)
            && !input_string.contains(FILE_NAME_EXT_VARIABLE_NAME)
            && !input_string.contains(FILE_EXT_VARIABLE_NAME)
        {
            return vec![input_string.to_string()];
        }
        for i in 0..self.memory.get(FILE_PATH_VARIABLE_NAME).unwrap().len() {
            let mut command = input_string.to_owned();
            if command.contains(FILE_NAME_VARIABLE_NAME)
                || command.contains(FILE_PATH_VARIABLE_NAME)
                || command.contains(FILE_DIR_VARIABLE_NAME)
                || command.contains(FILE_NAME_EXT_VARIABLE_NAME)
                || command.contains(FILE_EXT_VARIABLE_NAME)
            {
                if command.contains(FILE_NAME_VARIABLE_NAME) {
                    command = command.replace(
                        &["{", FILE_NAME_VARIABLE_NAME, "}"].concat(),
                        &self
                            .memory
                            .get(FILE_NAME_VARIABLE_NAME)
                            .unwrap()
                            .get(i)
                            .unwrap(),
                    );
                }
                if command.contains(FILE_PATH_VARIABLE_NAME) {
                    command = command.replace(
                        &["{", FILE_PATH_VARIABLE_NAME, "}"].concat(),
                        &self
                            .memory
                            .get(FILE_PATH_VARIABLE_NAME)
                            .unwrap()
                            .get(i)
                            .unwrap(),
                    );
                }
                if command.contains(FILE_DIR_VARIABLE_NAME) {
                    command = command.replace(
                        &["{", FILE_DIR_VARIABLE_NAME, "}"].concat(),
                        &self
                            .memory
                            .get(FILE_DIR_VARIABLE_NAME)
                            .unwrap()
                            .get(i)
                            .unwrap(),
                    );
                }
                if command.contains(FILE_NAME_EXT_VARIABLE_NAME) {
                    command = command.replace(
                        &["{", FILE_NAME_EXT_VARIABLE_NAME, "}"].concat(),
                        &self
                            .memory
                            .get(FILE_NAME_EXT_VARIABLE_NAME)
                            .unwrap()
                            .get(i)
                            .unwrap(),
                    );
                }
                if command.contains(FILE_EXT_VARIABLE_NAME) {
                    command = command.replace(
                        &["{", FILE_EXT_VARIABLE_NAME, "}"].concat(),
                        &self
                            .memory
                            .get(FILE_EXT_VARIABLE_NAME)
                            .unwrap()
                            .get(i)
                            .unwrap(),
                    );
                }
                modified_string.push(command);
            } else {
                return vec![command];
            }
        }

        modified_string
    }

    pub fn replace_placeholders(&mut self, template: &str) -> String {
        // Define a regular expression to match placeholders like {placeholder_name}
        let re = Regex::new(r#"\{([^{}]*)\}"#).unwrap();

        // Get user input for each placeholder
        let mut replacements: Vec<(String, String)> = Vec::new();
        for capture in re.captures_iter(template) {
            let placeholder = &capture[1];
            if self.exists(&placeholder.to_string()) {
                continue;
            }

            let trimmed_input = Logger::input_default(&["\nEnter ", placeholder, ":"].concat(), "forge");
            self.add(placeholder.to_owned(), &trimmed_input);
            replacements.push((format!("{{{}}}", placeholder), trimmed_input));
        }

        // Perform replacements in the template string
        let mut result = template.to_string();
        for (placeholder, replacement) in replacements {
            result = result.replace(&placeholder, &replacement);
        }

        result
    }
}
