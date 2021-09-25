mod environment;
mod util;

use environment::Environment;
use serenity::Client;
use util::logger::Logger;

#[tokio::main]
async fn main() {
    let env = Environment::load();

    let mut client = Client::builder(env.discord_token)
        .application_id(env.application_id)
        .await
        .expect("Could not create client.");

    Logger::notify("Bot is starting");
    client.start().await.expect("Could not start client.");
}
