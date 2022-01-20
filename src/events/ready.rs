use poise::serenity_prelude::{Context, Ready};

use crate::util::logger::Logger;

pub async fn handle(context: Context, ready: Ready) {
    let bot_tag = ready.user.tag();
    Logger::notify(&format!("{bot_tag} is online"));
    Logger::info("Awaiting update command");
    context.idle().await;
}
