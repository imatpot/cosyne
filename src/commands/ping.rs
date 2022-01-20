// Plagiarized from https://github.com/kangalioo/poise/blob/master/src/samples.rs

use poise::{command, say_reply};

use crate::types::poise::{Context, Error};

/// Check whether the bot is responsive.
#[command(rename = "ping", slash_command, prefix_command)]
pub async fn exec(context: Context<'_>) -> Result<(), Error> {
    say_reply(context, "Pong!").await?;
    Ok(())
}
