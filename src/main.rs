use crate::parser::{load_forge, yaml_to_object};

pub mod logging;
pub mod parser;
fn main() {
    println!("Hello, world!");
    let file_content = load_forge("./examples/forge.yaml");
    let data = yaml_to_object(&file_content);

    println!("{file_content}");
    println!("{:?}", data);
}
