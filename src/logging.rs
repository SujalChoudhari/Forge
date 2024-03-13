use colored::{ColoredString, Colorize};
use std::process::exit;

use crate::constants::{
    APP_NAME, APP_SUBTITLE, APP_VERSION, ERROR_TAG, INFORMATION_TAG, INPUT_TAG, WARNING_TAG,
};

#[derive(Debug)]
enum LogType {
    INPUT,
    INFO,
    WARN,
    ERROR,
}

pub static mut IS_VERBOSE: bool = false;

pub fn error(message: &str) {
    log(LogType::ERROR, message);
}

pub fn start() {
    let title = APP_NAME.truecolor(0, 0, 0).bold().on_bright_cyan();
    let version = if unsafe { IS_VERBOSE } {
        APP_VERSION.truecolor(100, 100, 100).italic()
    } else {
        "".bold()
    };
    let message = APP_SUBTITLE.truecolor(90, 90, 90).italic();
    println!(" {} {} {}\n", title, version, message);
}

pub fn warn(message: &str) {
    log(LogType::WARN, message);
}

pub fn info(message: &str) {
    log(LogType::INFO, message);
}

pub fn input(message: &str) {
    log(LogType::INPUT, message);
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
        LogType::INFO => INFORMATION_TAG
            .bold()
            .on_truecolor(0, 255, 0)
            .truecolor(0, 0, 0),
        LogType::INPUT => INPUT_TAG
            .bold()
            .on_truecolor(0, 150, 255)
            .truecolor(0, 0, 0),
        LogType::WARN => WARNING_TAG
            .bold()
            .on_truecolor(0, 255, 255)
            .truecolor(0, 0, 0),
        LogType::ERROR => ERROR_TAG.bold().on_truecolor(255, 0, 0).truecolor(0, 0, 0),
    };
    info_tag
}

fn log(log_type: LogType, message: &str) {
    let info_tag = get_log_tag(&log_type);
    let time_string: colored::ColoredString = if unsafe { IS_VERBOSE } {
        get_time().italic().magenta()
    } else {
        "".bold()
    };

    match log_type {
        LogType::INFO => {
            println!("{info_tag} {time_string}: {message}");
        }
        LogType::INPUT => {
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
    if unsafe { IS_VERBOSE } {
        println!("{info_tag} {time_string}:\n{title}\n{message}");
    } else {
        println!("{info_tag}: {title}");
    }
}
