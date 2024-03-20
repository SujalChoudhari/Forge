use argparser::Arguments;
use cli::help::print_help_message;
use collector::Collector;
use constants::*;
use engraver::Engraver;
use filehandler::update_last_modified_of_files;

use forger::Forger;
use logger::Logger;
use variable::Variables;

pub mod collector;
pub mod engraver;
pub mod forger;

#[derive(Debug)]
pub struct Application {
    arguments: Arguments,
    variables: Variables,
    collector: Collector,
    engraver: Engraver,
    forger: Forger,
}

impl Application {
    pub fn new() -> Self {
        // load arguments
        Application {
            arguments: Arguments::load_command_line_arguments(),
            variables: Variables::new(),
            collector: Collector::new(),
            engraver: Engraver::new(),
            forger: Forger::new(),
        }
    }

    pub fn run(&mut self) {
        // check for code should continue execution
        if self.process_flags_and_options() {
            Logger::start();
            self.collector.collect(&mut self.arguments, &mut self.variables);
            self.engraver
                .engrave(&self.arguments, &mut self.variables, &self.collector.job);
            self.forger
                .forge(&mut self.variables, &self.collector.commands_to_run);
            self.quench();
        };
    }

    /// Process the flags
    /// #### Returns:
    /// - [bool] :  `true` if the recipe can execute, `false` if code has to terminate.
    fn process_flags_and_options(&mut self) -> bool {
        // Non-Terminating Flags
        // verbose flag
        if self.arguments.is_flag_set(VERBOSE_FLAG) {
            Logger::set_is_verbose(true);
        }
        // force flag
        if self.arguments.is_flag_set(FORCE_EXECUTE_FLAG) {
            self.forger.is_force_execute_all = true
        }

        // Terminating Flags
        // version flag
        if self.arguments.is_flag_set(VERSION_FLAG) {
            Logger::info(&["Current forge version: ", APP_VERSION].concat());
            return false;
        }

        // help flag
        if self.arguments.is_flag_set(HELP_FLAG) {
            print_help_message();
            return false;
        }

        true
    }

    fn quench(&mut self) {
        if self.collector.is_recipe_updated() {
            update_last_modified_of_files(vec![APP_FILENAME]);
        }
        // set the time to 0 for changed files
        update_last_modified_of_files(self.collector.changed_file_paths.to_owned());
    }
}
