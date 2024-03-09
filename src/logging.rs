use std::process::exit;
use colored::Colorize;

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

fn get_time() -> String {
    let current_datetime = chrono::Local::now();
    let formatted_time = current_datetime.format("%H:%M:%S%.3f").to_string();
    formatted_time
}

fn log(log_type: LogType, message: &str) {
    let time_string = get_time().italic().magenta();

    let info_tag = match log_type {
        LogType::DEBUG => "[DEBG]".bold().bright_blue(),
        LogType::INFO =>"[INFO]".bold().bright_green(),
        LogType::WARN =>"[WARN]".bold().bright_yellow(),
        LogType::ERROR =>"[EROR]".bold().bright_red(),
    };

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
