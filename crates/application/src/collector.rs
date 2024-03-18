use argparser::Arguments;
use cli::handle_cli_command;
use constants::{
    APP_FILENAME, APP_FILENAME_DEFAULT_PATH, DEFALUT_DIR, DEFAULT_RECIPE, FILE_DIR_VARIABLE_NAME,
    FILE_EXT_VARIABLE_NAME, FILE_NAME_EXT_VARIABLE_NAME, FILE_NAME_VARIABLE_NAME,
    FILE_PATH_VARIABLE_NAME, LINUX_STRING, MAC_STRING, WIN_STRING,
};
use filehandler::{
    get_changed_files, get_files_in_directory_with_criteria, get_last_modified_of_files,
};
use filetime::FileTime;
use interpreter::{get_commands, get_dependencies, get_job, get_operating_systems, get_run_always};
use logger::Logger;
use parser::load_forge;
use std::{path::PathBuf, time::SystemTime};
use variable::Variables;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Collector {
    pub job: Yaml,
    pub is_forge_call: bool,
    pub changed_file_paths: Vec<PathBuf>,
    pub os: String,
    pub commands_to_run: Vec<String>,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            os: if cfg!(target_os = "windows") {
                WIN_STRING.to_string()
            } else if cfg!(target_os = "macos") {
                MAC_STRING.to_string()
            } else {
                LINUX_STRING.to_owned()
            },
            job: Yaml::Null,
            is_forge_call: true,
            changed_file_paths: vec![],
            commands_to_run: vec![],
        }
    }

    pub fn collect(&mut self, arguments: &Arguments, variables: &mut Variables) -> bool {
        self.attempt_cli_call(arguments);

        if !self.is_forge_call {
            return false;
        }

        self.get_changed_file_paths();
        let recipe_name = self.get_needed_recipe_and_update_job(arguments);
        self.check_if_run_is_necessary(recipe_name);

        if !self.is_forge_call {
            return false;
        }
        self.update_variables(variables);
        self.commands_to_run = get_commands(&self.job);
        return true;
    }

    pub fn is_recipe_updated(&self) -> bool {
        !FileTime::from_system_time(
            get_last_modified_of_files(&[&APP_FILENAME])
                .get(0)
                .unwrap_or(&SystemTime::now())
                .to_owned(),
        )
        .eq(&FileTime::from_unix_time(0, 0))
    }

    /// returns false if it was a cli call. and true if it was a forge recipe call.
    fn attempt_cli_call(&mut self, arguments: &Arguments) {
        if handle_cli_command(&arguments.nameless) {
            self.is_forge_call = false;
        };
    }

    fn get_needed_recipe_and_update_job(&mut self, arguments: &Arguments) -> String {
        // load forge file
        let all_reciepe = load_forge(APP_FILENAME_DEFAULT_PATH);

        // get the recipe name
        let recipe_name: String = if arguments.nameless.len() >= 2 {
            arguments.nameless.get(1).unwrap().to_string()
        } else {
            DEFAULT_RECIPE.to_string()
        };

        // extract the needed recipe
        if let Some(returned_job) = get_job(all_reciepe, recipe_name.to_owned()) {
            self.job = returned_job;
        } else {
            Logger::error(&["Recipe \"", &recipe_name, "\" does not exist."].concat())
        };

        recipe_name
    }

    fn get_changed_file_paths(&mut self) {
        // get the changed files
        let detectable_files_from_user = get_dependencies(&self.job);
        if self.is_recipe_updated() {
            self.changed_file_paths =
                get_files_in_directory_with_criteria(DEFALUT_DIR, &detectable_files_from_user);
        } else {
            self.changed_file_paths = get_changed_files(get_files_in_directory_with_criteria(
                DEFALUT_DIR,
                &detectable_files_from_user,
            ));
        }
    }

    fn check_if_run_is_necessary(&mut self, recipe_name: String) {
        // quit conditions
        if self.changed_file_paths.len() == 0 && !self.is_recipe_updated() {
            self.is_forge_call = false;
        }
        if !get_operating_systems(&self.job).contains(&self.os)
            && !get_operating_systems(&self.job).contains(&"all".to_string())
        {
            Logger::warn(
                &[
                    "\"",
                    &recipe_name,
                    "\" recipe is not for \"",
                    &self.os,
                    "\"",
                ]
                .concat(),
            );
            self.is_forge_call = false;
        }
        if get_run_always(&self.job) {
            self.is_forge_call = true;
        }
    }

    fn update_variables(&self, variables: &mut Variables) {
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

        variables.add_vec(FILE_PATH_VARIABLE_NAME.to_string(), relative_path);
        variables.add_vec(FILE_NAME_VARIABLE_NAME.to_string(), names);
        variables.add_vec(
            FILE_NAME_EXT_VARIABLE_NAME.to_string(),
            names_with_extension,
        );
        variables.add_vec(FILE_DIR_VARIABLE_NAME.to_string(), parent_directories);
        variables.add_vec(FILE_EXT_VARIABLE_NAME.to_string(), extensions);
    }
}
