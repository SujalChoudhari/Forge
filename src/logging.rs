use chrono::Utc;
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

fn get_time() -> String{
    let current_datetime = Utc::now();
    let formatted_time = current_datetime.format("%H:%M:%S%.3f").to_string();
    formatted_time
}

fn log(log_type: LogType, message: &str) {
    let time_string = get_time();
    match log_type {
        LogType::DEBUG => println!("[DEBG] {time_string}: {message}"),
        LogType::INFO => println!("[INFO] {time_string}: {message}"),
        LogType::WARN => println!("[WARN] {time_string}: {message}"),
        LogType::ERROR => println!("[EROR] {time_string}: {message}"),
    }
}
