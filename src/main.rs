use argparser::load_command_line_arguents;
use variables::{Variables};

use crate::{commander::execute, parser::load_forge};

pub mod argparser;
pub mod commander;
pub mod interpreter;
pub mod logging;
pub mod parser;
pub mod variables;

fn main() {
    let data = load_forge("./examples/forge");
    let job = interpreter::get_job(data, "forge".to_string());
    let vars: std::collections::HashMap<String, Vec<String>> =
        interpreter::get_variables(&job);
    let os = interpreter::get_operating_systems(&job);
    let deps = interpreter::get_dependencies(&job);
    let com = interpreter::get_commands(&job);
    println!("{:?}", vars);
    println!("{:?}", os);
    println!("{:?}", deps);
    println!("{:?}", com);
    load_command_line_arguents();
    let out = execute(&"tree ./src ".to_string());
    // println!("{}",out);

    let mut variables = Variables::new();

    variables.add(String::from("names"), String::from("John"));
    variables.add(String::from("names"), String::from("Alice"));
    variables.add_from_hash(&vars);
    let names = variables.get(&String::from("names"));
    match names {
        Some(values) => {
            println!("Values for 'names': {:?}", values);
        }
        None => println!("Key 'names' not found"),
    }

    let non_existent_key = String::from("non_existent_key");
    if variables.exists(&non_existent_key) {
        println!("Key '{}' exists", non_existent_key);
    } else {
        println!("Key '{}' does not exist", non_existent_key);
    }

    println!("{:?}",variables);
}
