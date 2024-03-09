use std::collections::HashMap;
#[derive(Debug)]
pub struct Arguments {
    nameless: Vec<String>,
    flags: Vec<String>,
    keword_arguments: HashMap<String, String>,
}

pub fn load_command_line_arguents() -> Arguments {
    let mut args = Arguments {
        nameless: Vec::new(),
        flags: Vec::new(),
        keword_arguments: HashMap::new(),
    };

    for individual_args in std::env::args() {
        if individual_args.starts_with("--") {
            // keyword
            if let Some(split_args) = individual_args.split_once("=") {
                args.keword_arguments.insert(
                    split_args.0.replace("--", "").to_owned(),
                    split_args.1.to_owned(),
                );
            }
        } else if individual_args.starts_with("-") {
            // flags
            args.flags.push(individual_args.replace("-", ""));
        } else {
            // nameless arg
            args.nameless.push(individual_args);
        }
    }

    println!("{:?}", args);

    args
}
