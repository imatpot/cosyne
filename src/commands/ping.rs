// Plagiarized from https://github.com/kangalioo/poise/blob/master/src/samples.rs

use chrono::Local;
use poise::{command, say_reply};

use crate::{
    types::poise::{Context, Error},
    util::logger::Logger,
};

/// Check whether the bot is responsive.
#[command(rename = "ping", slash_command, prefix_command)]
pub async fn exec(context: Context<'_>) -> Result<(), Error> {
    let latency = Local::now()
        .signed_duration_since(context.created_at())
        .num_milliseconds();

    say_reply(context, format!("Pong! It took me `{latency}ms` to react")).await?;

    Logger::debug(&format!("Latency: {latency}ms"));

    Ok(())
}
