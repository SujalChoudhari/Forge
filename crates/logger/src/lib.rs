use colored::{ColoredString, Colorize};
use constants::{
    APP_NAME, APP_SUBTITLE, APP_VERSION, ERROR_TAG, INFORMATION_TAG, INPUT_TAG, WARNING_TAG,
};
use std::io::{self, Write};

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
    let title = format!(
        " {} {} {}",
        APP_NAME.truecolor(0, 0, 0).bold().on_bright_cyan(),
        if unsafe { IS_VERBOSE } {
            APP_VERSION.truecolor(100, 100, 100).italic()
        } else {
            "".bold()
        },
        APP_SUBTITLE.truecolor(90, 90, 90).italic()
    );
    println!("{}\n", title);
}

pub fn warn(message: &str) {
    log(LogType::WARN, message);
}

pub fn info(message: &str) {
    log(LogType::INFO, message);
}

pub fn input(prompt: &str, default: &str) -> String {
    let info_tag = get_log_tag(&LogType::INPUT);
    let time_string = get_time_colored();
    print!(
        "{info_tag} {time_string}: {} ({}) >> ",
        prompt,
        default.truecolor(100, 100, 100)
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}

pub fn intermidiate_info(title: &str, message: &str) {
    intermidiate_log(
        LogType::INFO,
        title.white().bold(),
        message.truecolor(40, 40, 40),
    );
}

pub fn intermidiate_error(title: &str, message: &str) {
    intermidiate_log(
        LogType::ERROR,
        title.white().bold(),
        message.truecolor(150, 0, 0),
    );
}

fn get_time_colored() -> ColoredString {
    if unsafe { IS_VERBOSE } {
        get_time().italic().magenta()
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
        LogType::INPUT => (INPUT_TAG, (0, 150, 255)),
        LogType::WARN => (WARNING_TAG, (255, 255, 0)),
        LogType::ERROR => (ERROR_TAG, (255, 0, 0)),
    };
    tag.bold()
        .on_truecolor(color.0, color.1, color.2)
        .truecolor(0, 0, 0)
}

fn log(log_type: LogType, message: &str) {
    let info_tag = get_log_tag(&log_type);
    let time_string = get_time_colored();
    match log_type {
        LogType::INFO => println!(
            "{info_tag} {time_string}: {message}",
            info_tag = info_tag,
            time_string = time_string,
            message = message
        ),
        LogType::INPUT => (), // nothing to log for input
        LogType::WARN | LogType::ERROR => eprintln!(
            "{info_tag} {time_string}: {message}",
            info_tag = info_tag,
            time_string = time_string,
            message = message
        ),
    }
}

fn intermidiate_log(log_type: LogType, title: ColoredString, message: ColoredString) {
    let info_tag = get_log_tag(&log_type);
    let time_string = get_time_colored();
    if unsafe { IS_VERBOSE } {
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
