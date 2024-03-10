use std::{path::PathBuf, vec};

use yaml_rust::Yaml;

use crate::{
    argparser::{load_command_line_arguents, Arguments},
    commander::execute,
    filehandler::{
        get_changed_files, get_files_in_directory_with_criteria, update_last_modified_of_files,
    },
    interpreter::{get_commands, get_dependencies, get_job, get_operating_systems, get_variables},
    parser::load_forge,
    variables::Variables,
};

#[derive(Debug)]
pub struct Forger {
    arguments: Arguments,
    job: Yaml,
    changed_file_paths: Vec<PathBuf>,
    forge_file_path: String,
    variables: Variables,
    os: String,
    commands_to_run: Vec<String>,
    can_run_job: bool,
}

impl Forger {
    pub fn new() -> Self {
        // load arguments
        Forger {
            arguments: load_command_line_arguents(),
            forge_file_path: "./forge".to_string(),
            variables: Variables::new(),
            job: Yaml::Null,
            changed_file_paths: vec![],
            commands_to_run: vec![],
            can_run_job: true,
            os: if cfg!(target_os = "windows") {
                "Win".to_string()
            } else if cfg!(target_os = "macos") {
                "Mac".to_string()
            } else {
                "Linux".to_owned()
            },
        }
    }

    pub fn collect(&mut self) {
        // load forge file
        let all_reciepe = load_forge(&self.forge_file_path);

        // get the receipe name
        let mut receipe_name: String = "forge".to_string();
        if self.arguments.nameless.len() > 2 {
            receipe_name = self.arguments.nameless.get(1).unwrap().to_owned();
        }

        // extract the needed recipe
        self.job = get_job(all_reciepe, receipe_name);

        // get the changed files
        self.changed_file_paths = get_changed_files(get_files_in_directory_with_criteria(
            "./",
            &get_dependencies(&self.job),
        ));
        if self.changed_file_paths.len() == 0 {
            self.can_run_job = false;
        }

        // get the commands to run
        self.commands_to_run = get_commands(&self.job);

        if !get_operating_systems(&self.job).contains(&self.os)
            && !get_operating_systems(&self.job).contains(&"all".to_string())
        {
            self.can_run_job = false;
        }
    }
    pub fn engrave(&mut self) {
        if !self.can_run_job {
            return;
        };
        // save args in variables
        self.variables
            .add_from_hash(&self.arguments.keword_arguments);
        // save the declared in vars
        self.variables.add_from_hash(&get_variables(&self.job))
        // add the default args
        // TODO
    }
    pub fn forge(&mut self) {
        if !self.can_run_job {
            return;
        };
        // clean templates
        let mut current_os = self.os.to_owned();
        let mut cleaned_commands_to_run: Vec<String> = vec![];
        for command in &self.commands_to_run {
            if command == "Linux" {
                current_os = "Linux".to_string();
            } else if command == "Win" {
                current_os = "Win".to_string();
            } else if command == "Mac" {
                current_os = "Mac".to_string();
            } else {
                if current_os == self.os {
                    cleaned_commands_to_run.push(command.to_string());
                }
            }
        }

        // format templates
        self.commands_to_run = self
            .variables
            .format_templates(cleaned_commands_to_run.to_owned());

        // run code

        for command in &self.commands_to_run {
            let _out = execute(command);
        }
    }
    pub fn quench(&mut self) {
        if !self.can_run_job {
            return;
        };
        // set the time to 0 for changed files
        update_last_modified_of_files(self.changed_file_paths.to_owned());
    }
}
