use commander::execute;
use constants::{LINUX_STRING, MAC_STRING, WIN_STRING};
use logger::Logger;
use variable::Variables;

#[derive(Debug)]
pub struct Forger {
    os: String,
    cleaned_commands_to_run: Vec<String>,
    pub commands_to_run: Vec<String>,
    pub is_force_execute_all: bool,
}

impl Forger {
    pub fn new() -> Self {
        Forger {
            os: if cfg!(target_os = "windows") {
                WIN_STRING.to_string()
            } else if cfg!(target_os = "macos") {
                MAC_STRING.to_string()
            } else {
                LINUX_STRING.to_owned()
            },
            cleaned_commands_to_run: vec![],
            commands_to_run: vec![],
            is_force_execute_all: false,
        }
    }

    pub fn forge(&mut self, variables: &mut Variables, raw_commands_to_execute: &Vec<String>) {
        self.clean_templates(raw_commands_to_execute);
        self.commands_to_run = variables.format_templates(self.cleaned_commands_to_run.to_owned());
        self.run_commands();
    }

    fn clean_templates(&mut self, raw_commands_to_execute: &Vec<String>) {
        // clean templates
        let mut current_os = self.os.to_owned();
        for command in raw_commands_to_execute {
            if command.to_lowercase() == LINUX_STRING {
                current_os = LINUX_STRING.to_string();
            } else if command.to_lowercase() == WIN_STRING {
                current_os = WIN_STRING.to_string();
            } else if command.to_lowercase() == MAC_STRING {
                current_os = MAC_STRING.to_string();
            } else {
                if current_os == self.os {
                    self.cleaned_commands_to_run.push(command.to_string());
                }
            }
        }
    }

    fn run_commands(&mut self) {
        // run code
        for command in &self.commands_to_run {
            let out: Result<String, String> = execute(command);
            match out {
                Result::Ok(_) => {}
                Result::Err(_) => {
                    if self.is_force_execute_all == false {
                        Logger::warn("An error occured while running the command.\nStopping Execution.\nTo continue executing set use '-f'.");
                        return;
                    }
                }
            }
        }
    }
}
