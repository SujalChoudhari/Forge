use std::collections::{BTreeMap, HashMap};

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

    pub fn add(&mut self, key: String, value: String) {
        // Insert the key-value pair into the HashMap
        self.memory.entry(key).or_insert_with(Vec::new).push(value);
    }

    pub fn add_from_hash(&mut self, map: &HashMap<String, Vec<String>>) {
        for (key, values) in map {
            for value in values {
                self.add(key.clone(), value.clone());
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

    pub fn replace_key(&self, input_strings: Vec<String>, key_to_replace: &str) -> Vec<String> {
        let mut result = Vec::new();
        for input_string in input_strings {
            // Check if the key_to_replace is present in the input string
            if input_string.contains(key_to_replace) {
                // Iterate over each value of the key
                if let Some(values) = self.memory.get(key_to_replace) {
                    for value in values {
                        // Replace the key_to_replace with the actual value
                        let replaced_string =
                            input_string.replace(&["{", key_to_replace, "}"].concat(), value);
                        // Add the replaced string to the result vector
                        result.push(replaced_string);
                    }
                }
            } else {
                // If key_to_replace is not found, add the input string as is to the result
                result.push(input_string);
            }
        }

        result
    }

    pub fn replace_all_keys(&self, mut input_strings: Vec<String>) -> Vec<String> {
        // Create a BTreeMap to store keys sorted by the length of their values
        let mut sorted_keys: BTreeMap<usize, &String> = BTreeMap::new();

        // Populate the sorted_keys map with keys and their corresponding value lengths
        for (key, values) in &self.memory {
            if let Some(value) = values.get(0) {
                let len = value.len();
                sorted_keys.insert(len, key);
            }
        }

        // Iterate over sorted keys and apply replacements
        for (_, key) in sorted_keys {
            input_strings = self.replace_key(input_strings, key);
        }

        input_strings
    }
}
