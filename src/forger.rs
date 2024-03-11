use std::{fs::File, path::PathBuf, time::SystemTime, vec};

use filetime::FileTime;
use yaml_rust::Yaml;

use crate::{
    argparser::{load_command_line_arguents, Arguments},
    commander::execute,
    filehandler::{
        get_changed_files, get_files_in_directory_with_criteria, get_last_modified_of_files,
        update_last_modified_of_files,
    },
    interpreter::{get_commands, get_dependencies, get_job, get_operating_systems, get_variables},
    logging::info,
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
            forge_file_path: "./forge.yaml".to_string(),
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
        let receipe_name: String = if self.arguments.nameless.len() >= 2 {
            self.arguments.nameless.get(1).unwrap().to_string()
        } else {
            "forge.yaml".to_string()
        };

        // extract the needed recipe
        self.job = get_job(all_reciepe, receipe_name.to_owned());

        // get the changed files
        let detectable_files_from_user = get_dependencies(&self.job);
        if self.is_forge_updated() {
            self.changed_file_paths =
                get_files_in_directory_with_criteria("./", &detectable_files_from_user);
        } else {
            self.changed_file_paths = get_changed_files(get_files_in_directory_with_criteria(
                "./",
                &detectable_files_from_user,
            ));
        }

        // quit conditions
        if self.changed_file_paths.len() == 0 && !self.is_forge_updated() {
            self.can_run_job = false;
            return;
        }
        if !get_operating_systems(&self.job).contains(&self.os)
            && !get_operating_systems(&self.job).contains(&"all".to_string())
        {
            info(
                &[
                    "\"",
                    &receipe_name,
                    "\" receipe is not for \"",
                    &self.os,
                    "\"",
                ]
                .concat(),
            );
            self.can_run_job = false;
        }

        let names: Vec<String> = self
            .changed_file_paths
            .iter()
            .filter_map(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str().map(String::from))
            })
            .collect();

        // Vector for storing names with extensions
        let names_with_extension: Vec<String> = self
            .changed_file_paths
            .iter()
            .filter_map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str().map(String::from))
            })
            .collect();

        let extensions: Vec<String> = self
            .changed_file_paths
            .iter()
            .filter_map(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str().map(String::from))
            })
            .collect();

        // Vector for storing parent directories
        let parent_directories: Vec<String> = self
            .changed_file_paths
            .iter()
            .filter_map(|path| {
                path.parent()
                    .and_then(|parent| parent.to_str().map(String::from))
            })
            .collect();

        let relative_path: Vec<String> = self
            .changed_file_paths
            .iter()
            .map(|path| path.to_str().unwrap_or_default().to_string())
            .collect();

        self.variables
            .add_vec("filePath".to_string(), relative_path);
        self.variables.add_vec("fileName".to_string(), names);
        self.variables
            .add_vec("fileNameExt".to_string(), names_with_extension);
        self.variables
            .add_vec("fileDir".to_string(), parent_directories);
        self.variables.add_vec("fileExt".to_string(), extensions);

        // get the commands to run
        self.commands_to_run = get_commands(&self.job);
    }

    pub fn engrave(&mut self) {
        if !self.can_run_job {
            return;
        };
        // save args in variables
        self.variables
            .add_from_hash(&self.arguments.keword_arguments);
        // save the declared in vars
        self.variables.add_from_hash(&get_variables(&self.job));
        // add the default args
        self.variables.add("os".to_string(), self.os.to_string());
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
            let _out: Result<String, String> = execute(command);
        }
    }

    pub fn quench(&mut self) {
        if !self.can_run_job {
            return;
        };

        if self.is_forge_updated() {
            update_last_modified_of_files(vec!["forge.yaml"]);
        }
        // set the time to 0 for changed files
        update_last_modified_of_files(self.changed_file_paths.to_owned());
    }

    fn is_forge_updated(&self) -> bool {
        !FileTime::from_system_time(
            get_last_modified_of_files(&[&"forge.yaml"])
                .get(0)
                .unwrap()
                .to_owned(),
        )
        .eq(&FileTime::from_unix_time(0, 0))
    }
}
