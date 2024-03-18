use colored::{ColoredString, Colorize};
use constants::{
    APP_NAME, APP_SUBTITLE, APP_VERSION, ERROR_TAG, INFORMATION_TAG, WARNING_TAG,
};
use inquire::{validator, MultiSelect};
use inquire::{Autocomplete, Select, Text};
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::RwLock;
#[derive(Debug)]
enum LogType {
    INFO,
    WARN,
    ERROR,
}

pub struct Logger {
    pub is_verbose: bool,
}

lazy_static! {
    pub static ref INSTANCE: Arc<RwLock<Logger>> =
        Arc::new(RwLock::new(Logger { is_verbose: false }));
}

impl Logger {
    pub fn set_is_verbose(state: bool) {
        INSTANCE.write().unwrap().is_verbose = state;
    }

    pub fn get_is_verbose() -> bool {
        INSTANCE.read().unwrap().is_verbose
    }

    pub fn error(message: &str) {
        Logger::log(LogType::ERROR, message);
    }

    pub fn start() {
        let title = format!(
            " {} {} {}",
            APP_NAME.truecolor(0, 0, 0).bold().on_bright_cyan(),
            if Logger::get_is_verbose() {
                APP_VERSION.truecolor(100, 100, 100).italic()
            } else {
                "".bold()
            },
            APP_SUBTITLE.truecolor(90, 90, 90).italic()
        );
        println!("{}\n", title);
    }

    pub fn warn(message: &str) {
        Logger::log(LogType::WARN, message);
    }

    pub fn info(message: &str) {
        Logger::log(LogType::INFO, message);
    }

    pub fn input_autocomplete(prompt: &str, list: Vec<&str>) -> String {
        let auto_complete_list: Vec<String> =
            list.iter().map(|val| val.to_owned().to_string()).collect();

        // Create a custom autocomplete trait implementation
        #[derive(Clone)]
        struct CustomAutocomplete {
            suggestions: Vec<String>,
        }

        impl Autocomplete for CustomAutocomplete {
            fn get_suggestions(
                &mut self,
                input: &str,
            ) -> Result<Vec<String>, Box<(dyn std::error::Error + Send + Sync + 'static)>>
            {
                let res = self
                    .suggestions
                    .iter()
                    .filter(|s| s.starts_with(input))
                    .cloned()
                    .collect();
                return Result::Ok(res);
            }

            fn get_completion(
                &mut self,
                _: &str,
                _: Option<String>,
            ) -> Result<Option<String>, Box<(dyn std::error::Error + Send + Sync + 'static)>>
            {
                Result::Ok(None)
            }
        }

        let autocomplete = CustomAutocomplete {
            suggestions: auto_complete_list,
        };

        let result = Text::new(prompt)
            .with_validator(validator::ValueRequiredValidator::new("Cannot be empty"))
            .with_autocomplete(autocomplete)
            .prompt();

        match result {
            Ok(value) => value,
            Err(_) => "".to_string(), // Return empty string on error
        }
    }

    pub fn input_default(prompt: &str, default: &str) -> String {
        let result = Text::new(prompt).with_default(default).prompt();

        match result {
            Ok(value) => value,
            Err(_) => default.to_string(), // Return default value on error
        }
    }

    pub fn input_choice(prompt: &str, choices: Vec<&str>) -> String {
        let result = Select::new(prompt, choices).prompt();

        match result {
            Ok(value) => value.to_string(),
            Err(_) => "".to_string(), // Return empty string on error
        }
    }

    pub fn input_multiselect(prompt: &str, choices: Vec<&str>) -> Vec<String> {
        let result = MultiSelect::new(prompt, choices).prompt();

        match result {
            Ok(selections) => selections.iter().map(|string| string.to_string()).collect(),
            Err(_) => Vec::new(), // Return empty vector on error
        }
    }

    pub fn intermidiate_info(title: &str, message: &str) {
        Logger::intermidiate_log(
            LogType::INFO,
            title.white().bold(),
            message.truecolor(40, 40, 40),
        );
    }

    pub fn intermidiate_error(title: &str, message: &str) {
        Logger::intermidiate_log(
            LogType::ERROR,
            title.white().bold(),
            message.truecolor(150, 0, 0),
        );
    }

    fn get_time_colored() -> ColoredString {
        if Logger::get_is_verbose() {
            Logger::get_time().italic().magenta()
        } else {
            "".bold()
        }
    }

    fn get_time() -> String {
        let current_datetime = chrono::Local::now();
        current_datetime.format("%H:%M:%S%.3f").to_string()
    }

    fn get_log_tag(log_type: &LogType) -> ColoredString {
        let (tag, color) = match log_type {
            LogType::INFO => (INFORMATION_TAG, (0, 255, 0)),
            LogType::WARN => (WARNING_TAG, (255, 255, 0)),
            LogType::ERROR => (ERROR_TAG, (255, 0, 0)),
        };
        tag.bold()
            .on_truecolor(color.0, color.1, color.2)
            .truecolor(0, 0, 0)
    }

    fn log(log_type: LogType, message: &str) {
        let info_tag = Logger::get_log_tag(&log_type);
        let time_string = Logger::get_time_colored();
        match log_type {
            LogType::INFO => println!(
                "{info_tag} {time_string}: {message}",
                info_tag = info_tag,
                time_string = time_string,
                message = message
            ),
            LogType::WARN | LogType::ERROR => eprintln!(
                "{info_tag} {time_string}: {message}",
                info_tag = info_tag,
                time_string = time_string,
                message = message
            ),
        }
    }

    fn intermidiate_log(log_type: LogType, title: ColoredString, message: ColoredString) {
        let info_tag = Logger::get_log_tag(&log_type);
        let time_string = Logger::get_time_colored();
        if Logger::get_is_verbose() {
            println!(
                "{info_tag} {time_string}:\n{title}\n{message}",
                info_tag = info_tag,
                time_string = time_string,
                title = title,
                message = message
            );
        } else {
            println!("{info_tag}: {title}", info_tag = info_tag, title = title);
        }
    }
}
