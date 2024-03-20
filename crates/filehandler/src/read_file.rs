use logger::Logger;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            Logger::error(
                &[
                    "Error opening file: ",
                    file_path,
                    ": ",
                    error.to_string().as_str(),
                ]
                .concat(),
            );
            return String::new();
        }
    };

    let mut file_contents = String::new();
    if let Err(error) = file.read_to_string(&mut file_contents) {
        Logger::error(
            &[
                "Error reading file: ",
                file_path,
                ": ",
                error.to_string().as_str(),
            ]
            .concat(),
        );
        return String::new();
    }

    file_contents
}
