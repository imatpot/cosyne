use std::env;

use poise::serenity_prelude::validate_token;
use regex::Regex;

use crate::util::logger::LogLevel;

/// Contains read & validated environment variables.
pub struct Environment {
    /// Usually your bot's user id.
    pub application_id: u64,

    /// Token to connect to the Discord API.
    pub discord_token: String,

    /// Verbosity of logging
    pub log_level: LogLevel,
}

impl Environment {
    /// Fetches and validates required environment variables and returns them in an `Environment` struct.
    ///
    /// # Errors
    ///
    /// Errors if an environment variable is missing or in an invalid format.
    pub fn load() -> Environment {
        Environment {
            discord_token: get_discord_token(),
            application_id: get_application_id(),
            log_level: get_log_level(),
        }
    }
}

/// Returns an error message for a missing environment variable called `name`.
fn missing_variable(name: &str) -> String {
    format!("Missing environment variable {}", name)
}

/// Returns an error message for an invalid environment variable called `name`.
fn invalid_variable(name: &str) -> String {
    format!("Invalid environment variable {}", name)
}

/// Fetches and validates the application ID environment variable.
///
/// # Errors
///
/// Errors if `APPLICATION_ID` is missing has an invalid format.
fn get_application_id() -> u64 {
    let env_name = "APPLICATION_ID";
    let application_id_string = env::var(env_name).expect(&missing_variable(env_name));

    let is_18_digits = Regex::new(r"[0-9]{18}").unwrap();
    if !is_18_digits.is_match(&application_id_string) {
        panic!("{}", invalid_variable(env_name));
    }

    let application_id: u64 = application_id_string
        .parse()
        .expect(&invalid_variable(env_name));

    application_id
}

/// Fetches and validates the Discord API token environment variable.
///
/// # Errors
///
/// Errors if `DISCORD_TOKEN` is missing has an invalid format.
fn get_discord_token() -> String {
    let env_name = "DISCORD_TOKEN";
    let discord_token = env::var(env_name).expect(&missing_variable(env_name));

    if validate_token(&discord_token).is_err() {
        panic!("{}", invalid_variable(env_name));
    }

    discord_token
}

/// Fetches and validates the log environment variable.
///
/// # Errors
///
/// Errors if `LOG_LEVEL` is missing has an invalid format.
fn get_log_level() -> LogLevel {
    let env_name = "LOG_LEVEL";
    let log_level_value = env::var(env_name).expect(&missing_variable(env_name));
    let log_level = LogLevel::from_string(&log_level_value);

    match log_level {
        Ok(level) => return level,
        Err(_) => panic!("{}", invalid_variable(env_name)),
    }
}
