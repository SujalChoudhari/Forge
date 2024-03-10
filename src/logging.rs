use colored::{ColoredString, Colorize};
use std::process::exit;

#[derive(Debug)]
enum LogType {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub fn error(message: &str) {
    log(LogType::ERROR, message);
}

pub fn warn(message: &str) {
    log(LogType::WARN, message);
}

pub fn debug(message: &str) {
    log(LogType::DEBUG, message);
}

pub fn info(message: &str) {
    log(LogType::INFO, message);
}

pub fn intermidiate_info(title: &str, message: &str) {
    let content = message.truecolor(40, 40, 40);
    intermidiate_log(LogType::INFO, title.white().bold(), content);
}

pub fn intermidiate_error(title: &str, message: &str) {
    let content = message.truecolor(150, 0, 0);
    intermidiate_log(LogType::ERROR, title.white().bold(), content);
}

fn get_time() -> String {
    let current_datetime = chrono::Local::now();
    let formatted_time = current_datetime.format("%H:%M:%S%.3f").to_string();
    formatted_time
}

fn get_log_tag(log_type: &LogType) -> ColoredString {
    let info_tag = match log_type {
        LogType::DEBUG => "[DEBG]".bold().bright_blue(),
        LogType::INFO => "[INFO]".bold().bright_green(),
        LogType::WARN => "[WARN]".bold().bright_yellow(),
        LogType::ERROR => "[EROR]".bold().bright_red(),
    };
    info_tag
}

fn log(log_type: LogType, message: &str) {
    let time_string: colored::ColoredString = get_time().italic().magenta();
    let info_tag = get_log_tag(&log_type);
    match log_type {
        LogType::DEBUG => {
            println!("{info_tag} {time_string}: {message}");
        }
        LogType::INFO => {
            println!("{info_tag} {time_string}: {message}");
        }
        LogType::WARN => {
            eprintln!("{info_tag} {time_string}: {message}");
        }
        LogType::ERROR => {
            eprintln!("{info_tag} {time_string}: {message}");
            exit(0);
        }
    }
}

fn intermidiate_log(log_type: LogType, title: ColoredString, message: ColoredString) {
    let info_tag = get_log_tag(&log_type);
    let time_string: colored::ColoredString = get_time().italic().magenta();
    println!("{info_tag} {time_string}:\n{title}\n{message}");
}
