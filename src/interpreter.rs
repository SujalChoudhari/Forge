use crate::{
    constants::{
        ALWAYS_KEY, COMMANDS_KEY, DEFAULT_DETECT_PATTERN, DETECT_KEY, LINUX_STRING, MAC_STRING,
        OS_KEY, VARIABLES_KEY, WIN_STRING,
    },
    logging::error,
};
use std::{collections::HashMap, vec};
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
    if let Some(result) = get_list_or_string(yaml, OS_KEY.to_string()) {
        result.iter().map(|str| str.to_lowercase()).collect()
    } else {
        vec![
            LINUX_STRING.to_string(),
            WIN_STRING.to_string(),
            MAC_STRING.to_string(),
        ]
    }
}

pub fn get_dependencies(yaml: &Yaml) -> Vec<String> {
    if let Some(result) = get_list_or_string(yaml, DETECT_KEY.to_string()) {
        result
    } else {
        vec![DEFAULT_DETECT_PATTERN.to_string()]
    }
}

pub fn get_run_always(yaml: &Yaml) -> bool {
    match yaml {
        Yaml::Hash(map) => {
            if map.contains_key(&Yaml::String(ALWAYS_KEY.to_owned())) {
                let value_map = map[&Yaml::String(ALWAYS_KEY.to_owned())].to_owned();
                match value_map {
                    // if value map exists
                    Yaml::Boolean(boolean) => {
                        return boolean;
                    }
                    _ => error(
                        &[
                            "Non Boolean found in \"",
                            &ALWAYS_KEY,
                            "\", expected 'true' or 'false'",
                        ]
                        .concat(),
                    ),
                }
            } else {
                return false;
            }
        }
        _ => {
            return false;
        }
    };
    false
}

pub fn get_commands(yaml: &Yaml) -> Vec<String> {
    if let Some(result) = get_list_or_string(yaml, COMMANDS_KEY.to_string()) {
        result
    } else {
        vec![]
    }
}

pub fn get_variables(yaml: &Yaml) -> HashMap<String, Vec<String>> {
    if let Some(result) = get_key_value_pairs(yaml, VARIABLES_KEY.to_string()) {
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
