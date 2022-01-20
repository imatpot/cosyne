use poise::serenity_prelude::{Context, Ready};

use crate::util::logger::Logger;

pub fn handle(_context: Context, ready: Ready) {
    let bot_tag = ready.user.tag();
    Logger::notify(&format!("{bot_tag} is online"));
}
