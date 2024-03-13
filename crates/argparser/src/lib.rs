use std::collections::HashMap;

#[derive(Debug)]
/// Container to store all the arguments entered.
///
/// ### Properties
/// - `nameless`: `Vec<String>` - A vector to store nameless arguments.
/// - `flags`: `Vec<String>` - A vector to store flags.
/// - `keyword_arguments`: `HashMap<String, Vec<String>>` - A hashmap to store keyword arguments.
pub struct Arguments {
    pub nameless: Vec<String>,
    pub flags: Vec<String>,
    pub keyword_arguments: HashMap<String, Vec<String>>,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            nameless: Vec::new(),
            flags: Vec::new(),
            keyword_arguments: HashMap::new(),
        }
    }

    /// Parses command line arguments using [std::env::args].
    ///
    /// #### Complexity
    /// - O(n) where `n` is the number of command line arguments.
    ///
    /// #### Returns
    /// - [Arguments]: An instance of the [Arguments] struct containing parsed command line arguments.
    pub fn load_command_line_arguments() -> Arguments {
        let mut args = Arguments::new();

        for individual_arg in std::env::args() {
            if individual_arg.starts_with("--") {
                // keyword argument
                match individual_arg.split_once("=") {
                    Some((keyword, value)) => {
                        args.keyword_arguments
                            .entry(keyword.trim_start_matches("--").to_owned())
                            .or_insert_with(Vec::new)
                            .push(value.to_owned());
                    }
                    None => {
                        args.flags
                            .push(individual_arg.trim_start_matches("--").to_owned());
                    }
                }
            } else if individual_arg.starts_with("-") {
                // flag
                args.flags
                    .push(individual_arg.trim_start_matches("-").to_owned());
            } else {
                // nameless argument
                args.nameless.push(individual_arg);
            }
        }
        args
    }

    pub fn is_flag_set(&self, flag: (&str, &str)) -> bool {
        self.flags.contains(&flag.0.to_string()) || self.flags.contains(&flag.1.to_string())
    }
}
