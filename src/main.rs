mod commands;
mod environment;
mod events;
mod types;
mod util;

use environment::Environment;
use poise::{Framework, FrameworkOptions};
use util::logger::Logger;

use crate::events::Events;

#[tokio::main]
async fn main() {
    Logger::info("Loading environment...");
    let env = Environment::load();

    Logger::info("Setting up...");
    let framework = Framework::build()
        .token(env.discord_token)
        .prefix("cos")
        .user_data_setup(|_context, _ready, _framework| Box::pin(async { Ok(()) }))
        .client_settings(|client| client.raw_event_handler(Events))
        .options(FrameworkOptions {
            ..Default::default()
        })
        .command(commands::register::execute(), |f| f)
        .command(commands::ping::execute(), |f| f);

    Logger::info("Starting bot...");
    match framework.run().await {
        Ok(_) => Logger::debug("How/why did this log o_O"),
        Err(error) => Logger::panic(&format!("Failed to srart: {}", error)),
    }
}
