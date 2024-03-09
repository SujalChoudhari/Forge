extern crate yaml_rust;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

use crate::logging::error;

pub fn load_forge(file_path: &str) -> String {
    let res: Result<String, std::io::Error> = fs::read_to_string(file_path);
    let data: String;
    match res {
        Ok(val) => {
            data = val;
        }
        Err(_) => {
            error("Cannot read Forge file.");
            data = String::new();
        }
    };

    data
}

pub fn yaml_to_object(file_contents: &String) -> Yaml {
    let content = YamlLoader::load_from_str(file_contents);
    let data;
    match content {
        Ok(val) => {
            data = val[0].clone();
        }
        Err(_) => {
            error("Cannot Parse data as Yaml");
            data = Yaml::Null;
        }
    };
    data
}

pub fn load_yaml_from_filename(filename: &str) -> Yaml {
    let contents = load_forge(filename);
    yaml_to_object(&contents)
}
