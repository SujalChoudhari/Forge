use std::{path::PathBuf, time::SystemTime};

use argparser::Arguments;
use cli::{handle_cli_command, help::print_help_message};
use commander::execute;
use constants::*;
use filehandler::{
    get_changed_files, get_files_in_directory_with_criteria, get_last_modified_of_files,
    update_last_modified_of_files,
};
use filetime::FileTime;
use interpreter::{
    get_commands, get_dependencies, get_job, get_operating_systems, get_run_always, get_variables,
};
use logger::{error, info, start, warn, IS_VERBOSE};
use parser::load_forge;
use variable::Variables;
use yaml_rust::Yaml;

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
pub static mut IS_FORCE_EXECUTE_ALL: bool = false;

impl Forger {
    pub fn new() -> Self {
        // load arguments
        Forger {
            arguments: Arguments::load_command_line_arguments(),
            forge_file_path: APP_FILENAME_DEFAULT_PATH.to_string(),
            variables: Variables::new(),
            job: Yaml::Null,
            changed_file_paths: vec![],
            commands_to_run: vec![],
            can_run_job: true,
            os: if cfg!(target_os = "windows") {
                WIN_STRING.to_string()
            } else if cfg!(target_os = "macos") {
                MAC_STRING.to_string()
            } else {
                LINUX_STRING.to_owned()
            },
        }
    }

    pub fn run(&mut self) {
        // check for code should continue execution
        if !self.handle_flags() {
            return;
        };
        start();
        self.collect();
        self.engrave();
        self.forge();
        self.quench();
    }

    fn handle_flags(&mut self) -> bool {
        // Non-Terminating Flags
        // verbose flag
        if self.arguments.is_flag_set(VERBOSE_FLAG) {
            unsafe { IS_VERBOSE = true };
        }
        // force flag
        if self.arguments.is_flag_set(FORCE_EXECUTE_FLAG) {
            unsafe { IS_FORCE_EXECUTE_ALL = true }
        }

        // Terminating Flags
        // version flag
        if self.arguments.is_flag_set(VERSION_FLAG) {
            info(&["Current forge version: ", APP_VERSION].concat());
            return false;
        }

        // help flag
        if self.arguments.is_flag_set(HELP_FLAG) {
            print_help_message();
            return false;
        }

        true
    }

    fn collect(&mut self) {
        // check for recipes, is cli handled?
        if handle_cli_command(&self.arguments.nameless) {
            self.can_run_job = false;
            return;
        };

        // load forge file
        let all_reciepe = load_forge(&self.forge_file_path);

        // get the recipe name
        let recipe_name: String = if self.arguments.nameless.len() >= 2 {
            self.arguments.nameless.get(1).unwrap().to_string()
        } else {
            DEFAULT_RECIPE.to_string()
        };

        // extract the needed recipe
        if let Some(returned_job) = get_job(all_reciepe, recipe_name.to_owned()) {
            self.job = returned_job;
        } else {
            error(&["Recipe \"", &recipe_name, "\" does not exist."].concat())
        };

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

        // quit conditions
        if self.changed_file_paths.len() == 0 && !self.is_recipe_updated() {
            self.can_run_job = false;
        }
        if !get_operating_systems(&self.job).contains(&self.os)
            && !get_operating_systems(&self.job).contains(&"all".to_string())
        {
            warn(
                &[
                    "\"",
                    &recipe_name,
                    "\" recipe is not for \"",
                    &self.os,
                    "\"",
                ]
                .concat(),
            );
            self.can_run_job = false;
        }
        if get_run_always(&self.job) {
            self.can_run_job = true;
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
            .add_vec(FILE_PATH_VARIABLE_NAME.to_string(), relative_path);
        self.variables
            .add_vec(FILE_NAME_VARIABLE_NAME.to_string(), names);
        self.variables.add_vec(
            FILE_NAME_EXT_VARIABLE_NAME.to_string(),
            names_with_extension,
        );
        self.variables
            .add_vec(FILE_DIR_VARIABLE_NAME.to_string(), parent_directories);
        self.variables
            .add_vec(FILE_EXT_VARIABLE_NAME.to_string(), extensions);

        // get the commands to run
        self.commands_to_run = get_commands(&self.job);
    }

    fn engrave(&mut self) {
        if !self.can_run_job {
            return;
        };
        // save args in variables
        self.variables
            .add_from_hash(&self.arguments.keyword_arguments);
        // save the declared in vars
        self.variables.add_from_hash(&get_variables(&self.job));
        // add the default args
        self.variables
            .add(OS_VARIABLE_NAME.to_string(), self.os.to_string());
    }

    fn forge(&mut self) {
        if !self.can_run_job {
            return;
        };
        // clean templates
        let mut current_os = self.os.to_owned();
        let mut cleaned_commands_to_run: Vec<String> = vec![];
        for command in &self.commands_to_run {
            if command.to_lowercase() == LINUX_STRING {
                current_os = LINUX_STRING.to_string();
            } else if command.to_lowercase() == WIN_STRING {
                current_os = WIN_STRING.to_string();
            } else if command.to_lowercase() == MAC_STRING {
                current_os = MAC_STRING.to_string();
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
            let out: Result<String, String> = execute(command);
            match out {
                Result::Ok(_) => {}
                Result::Err(_) => {
                    if unsafe { IS_FORCE_EXECUTE_ALL == false } {
                        warn("An error occured while running the command.\nStopping Execution.\nTo continue executing set use '-f'.");
                        return;
                    }
                }
            }
        }
    }

    fn quench(&mut self) {
        if !self.can_run_job {
            return;
        };

        if self.is_recipe_updated() {
            update_last_modified_of_files(vec![APP_FILENAME]);
        }
        // set the time to 0 for changed files
        update_last_modified_of_files(self.changed_file_paths.to_owned());
    }

    fn is_recipe_updated(&self) -> bool {
        !FileTime::from_system_time(
            get_last_modified_of_files(&[&APP_FILENAME])
                .get(0)
                .unwrap_or(&SystemTime::now())
                .to_owned(),
        )
        .eq(&FileTime::from_unix_time(0, 0))
    }
}
