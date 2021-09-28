#![allow(dead_code)]

use chrono::Local;
use colored::{ColoredString, Colorize};

use crate::environment::Environment;

/// Verbosity of the logging.
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    NOTIFICATION,
    ERROR,
    PANIC,
}

impl LogLevel {
    /// Returns the first letter of the `LogLevel` in an appropriate color.
    fn colored_abbr(&self) -> ColoredString {
        match self {
            LogLevel::DEBUG => "[D]".purple(),
            LogLevel::INFO => "[I]".blue(),
            LogLevel::WARNING => "[W]".yellow(),
            LogLevel::NOTIFICATION => "[N]".green(),
            LogLevel::ERROR => "[E]".red(),
            LogLevel::PANIC => "[P]".red(),
        }
    }

    /// Converts the `LogLevel` into an `u8`.
    fn numeric(&self) -> u8 {
        match self {
            LogLevel::DEBUG => 0,
            LogLevel::INFO => 1,
            LogLevel::WARNING => 2,
            LogLevel::NOTIFICATION => 3,
            LogLevel::ERROR => 4,
            LogLevel::PANIC => 5,
        }
    }

    /// Returns a `LogLevel` matching `string`.
    ///
    /// # Errors
    ///
    /// Errors when no `LogLevel` matches `string`.
    pub fn from_string(string: &str) -> Result<Self, String> {
        match string {
            "DEBUG" => Ok(LogLevel::DEBUG),
            "INFO" => Ok(LogLevel::INFO),
            "WARNING" => Ok(LogLevel::WARNING),
            "NOTIFICATION" => Ok(LogLevel::NOTIFICATION),
            "ERROR" => Ok(LogLevel::ERROR),
            "PANIC" => Ok(LogLevel::PANIC),

            _ => Err(format!("LogLevel::{} doesn't exist", string)),
        }
    }
}

pub struct Logger;

impl Logger {
    /// Log `content` with a timestamp and appropriate `log_level`.
    fn log(log_level: LogLevel, content: &str) {
        if Logger::should_log(&log_level) {
            let now = Local::now().format("[%Y-%m-%d %H:%M:%S]");
            let level = log_level.colored_abbr();

            if log_level.numeric() > 2 {
                println!("-------------------------");
                println!("{} {} {}", now, level, content);
                println!("-------------------------");
            } else {
                println!("{} {} {}", now, level, content);
            }
        }
    }

    /// Comapres `log_level` to the corresponding environment variable.
    fn should_log(log_level: &LogLevel) -> bool {
        log_level.numeric() >= Environment::load().log_level.numeric()
    }

    /// Log `content` as a debug value.
    pub fn debug(content: &str) {
        Logger::log(LogLevel::DEBUG, content);
    }

    /// Log `content` as an information.
    pub fn info(content: &str) {
        Logger::log(LogLevel::INFO, content);
    }

    /// Log `content` as a warning.
    pub fn warn(content: &str) {
        Logger::log(LogLevel::WARNING, content);
    }

    /// Log `content` as an error.
    pub fn error(content: &str) {
        Logger::log(LogLevel::ERROR, content);
    }

    /// Log `content` as a notification.
    pub fn notify(content: &str) {
        Logger::log(LogLevel::NOTIFICATION, content);
    }

    pub fn panic(content: &str) {
        Logger::log(LogLevel::PANIC, content);
        std::process::exit(1);
    }
}
