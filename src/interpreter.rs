use std::{collections::HashMap, vec};
use crate::logging::error;
use yaml_rust::Yaml;

pub fn get_job(yaml: Yaml, job_name: String) -> Yaml {
    let result = match yaml {
        Yaml::Hash(map) => {
            if map.contains_key(&Yaml::String(job_name.to_owned())) {
                map[&Yaml::String(job_name.to_owned())].clone()
            } else {
                error(&["Job \"", &job_name, "\" does not exist."].concat());
                Yaml::Null
            }
        }
        _ => {
            error("Forge is empty.");
            Yaml::Null
        }
    };
    result
}

pub fn get_operating_systems(yaml: &Yaml) -> Vec<String> {
    if let Some(result) = get_list_or_string(yaml, "on".to_string()) {
        result
    } else {
        vec!["Linux".to_string(), "Win".to_string(), "Mac".to_string()]
    }
}

pub fn get_dependencies(yaml: &Yaml) -> Vec<String> {
    if let Some(result) = get_list_or_string(yaml, "detect".to_string()) {
        result
    } else {
        vec!["*".to_string()]
    }
}

pub fn get_commands(yaml: &Yaml) -> Vec<String> {
    if let Some(result) = get_list_or_string(yaml, "run".to_string()) {
        result
    } else {
        vec![]
    }
}

pub fn get_variables(yaml: &Yaml) -> HashMap<String, Vec<String>> {
    if let Some(result) = get_key_value_pairs(yaml, "vars".to_string()) {
        result
    } else {
        let result: HashMap<String, Vec<String>> = HashMap::new();
        result
    }
}

fn get_key_value_pairs(yaml: &Yaml, keyword: String) -> Option<HashMap<String, Vec<String>>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    match yaml {
        Yaml::Hash(map) => {
            if map.contains_key(&Yaml::String(keyword.to_owned())) {
                let variable_map = map[&Yaml::String(keyword.to_owned())].to_owned();
                match variable_map {
                    // if variable map exists
                    Yaml::Hash(hashmap) => {
                        for (key, value) in hashmap {
                            let variable_key = match key {
                                Yaml::String(raw_str) => raw_str,
                                _ => {
                                    error(&["Non String key found in \"", &keyword, "\""].concat());
                                    "".to_string()
                                }
                            };
                            let variable_value = match value {
                                Yaml::String(raw_str) => vec![raw_str],
                                Yaml::Array(array) => {
                                    let mut value_array: Vec<String> = Vec::new();
                                    for value in array {
                                        match value {
                                            Yaml::String(str) => value_array.push(str),
                                            _ => error(
                                                &[
                                                    "Non String found in Matrix in \"",
                                                    &variable_key,
                                                    "\" in\"",
                                                    &keyword,
                                                    "\"",
                                                ]
                                                .concat(),
                                            ),
                                        }
                                    }
                                    value_array
                                }
                                _ => {
                                    error(
                                        &[
                                            "Non String found in\"",
                                            &variable_key,
                                            "\" in\"",
                                            &keyword,
                                            "\"",
                                        ]
                                        .concat(),
                                    );
                                    vec![]
                                }
                            };
                            result.insert(variable_key, variable_value);
                        }
                    }
                    _ => error(&["Non Object found in \"", &keyword, "\""].concat()),
                }
            } else {
                return Option::None;
            }
        }
        _ => {
            return Option::None;
        }
    };

    Option::Some(result)
}

fn get_list_or_string(yaml: &Yaml, keyword: String) -> Option<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    match yaml {
        Yaml::Hash(map) => {
            if map.contains_key(&Yaml::String(keyword.to_owned())) {
                let value_map = map[&Yaml::String(keyword.to_owned())].to_owned();
                match value_map {
                    // if value map exists
                    Yaml::String(str) => result.push(str),
                    Yaml::Array(array) => {
                        for value in array {
                            match value {
                                Yaml::String(raw_str) => result.push(raw_str),
                                _ => error(
                                    &["Non String found in Matrix in \"", &keyword, "\""].concat(),
                                ),
                            }
                        }
                    }
                    _ => error(&["Non String found in \"", &keyword, "\""].concat()),
                }
            } else {
                return Option::None;
            }
        }
        _ => {
            return Option::None;
        }
    };
    Option::Some(result)
}
