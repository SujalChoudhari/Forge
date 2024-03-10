use std::collections::HashMap;

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
}
