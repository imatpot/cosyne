mod commands;
mod environment;
mod events;
mod types;
mod util;

use std::collections::HashSet;

use crate::environment::Environment;
use crate::events::Events;
use crate::util::logger::Logger;

use poise::{
    serenity_prelude::UserId, Context, Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions,
};
use types::poise::{Data, Error};

#[tokio::main]
async fn main() {
    Logger::info("Loading environment");
    let env = Environment::load();

    Logger::info("Setting up");

    let owners = HashSet::from([UserId(324943524132814849)]);
    let prefixes = vec![
        Prefix::Literal("cosyne"),
        Prefix::Literal("cos"),
        Prefix::Literal("c"),
    ];

    let framework_options = FrameworkOptions {
        owners,

        pre_command: |context| Box::pin(log_command_invocation(context)),

        prefix_options: PrefixFrameworkOptions {
            additional_prefixes: prefixes,
            mention_as_prefix: true,
            case_insensitive_commands: true,

            ..PrefixFrameworkOptions::default()
        },

        ..FrameworkOptions::default()
    };

    let framework = Framework::build()
        .token(env.discord_token)
        .options(framework_options)
        .user_data_setup(|_, __, ___| Box::pin(async { Ok(()) }))
        .client_settings(|client| client.raw_event_handler(Events))
        .command(commands::update::exec(), |f| f)
        .command(commands::ping::exec(), |f| f);

    Logger::info("Starting bot");
    match framework.run().await {
        Ok(_) => Logger::panic("Bot shut down unexpectedly"),
        Err(error) => Logger::panic(&format!("Failed to start: {error}")),
    }
}

async fn log_command_invocation(context: Context<'_, Data, Error>) {
    let author = context.author().tag();
    let command = context.command().unwrap().name();
    let guild = context.guild().unwrap().name;

    Logger::info(&format!("{author} ran [{command}] in {guild}"));
}
