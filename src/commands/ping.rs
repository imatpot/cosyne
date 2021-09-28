// Plagiarized from https://github.com/kangalioo/poise/blob/master/src/samples.rs

use poise::say_reply;

use crate::{
    types::poise::{Context, Error},
    util::logger::Logger,
};

/// Check whether the bot is responsive.
#[poise::command(rename = "ping", slash_command)]
pub async fn execute(context: Context<'_>) -> Result<(), Error> {
    Logger::info(&format!(
        "{} ran [ping] in {}",
        context.author().tag(),
        context.guild().unwrap().name
    ));

    say_reply(context, "Pong!").await?;

    Ok(())
}
