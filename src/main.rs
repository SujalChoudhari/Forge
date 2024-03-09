use crate::parser::load_forge;

pub mod interpreter;
pub mod logging;
pub mod parser;
fn main() {
    let data = load_forge("./examples/forge");
    let job = interpreter::get_job(data, "build".to_string());
    let variables: std::collections::HashMap<String, Vec<String>> = interpreter::get_variables(&job);
    let os = interpreter::get_operating_systems(&job);
    let deps = interpreter::get_dependencies(&job);
    let com = interpreter::get_commands(&job);
    println!("{:?}",variables);
    println!("{:?}",os);
    println!("{:?}",deps);
    println!("{:?}",com);
}
