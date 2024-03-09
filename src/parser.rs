extern crate yaml_rust;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

use crate::logging::error;

pub fn load_forge(filename: &str) -> Yaml {
    let contents = load_contents_of_file(filename);
    parse_string_into_objects(&contents)
}

fn load_contents_of_file(file_path: &str) -> String {
    let res: Result<String, std::io::Error> = fs::read_to_string(file_path);
    let data: String;
    match res {
        Ok(val) => {
            data = val.to_string();
            if val.len() > 0 {
            } else {
                error("Forge is empty");
            }
        }
        Err(_) => {
            let printable_path = file_path;
            error(&["Cannot open the \"", printable_path, "\" file"].concat());
            data = String::new();
        }
    };

    data
}

fn parse_string_into_objects(file_contents: &String) -> Yaml {
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
