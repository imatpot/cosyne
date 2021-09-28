use poise::serenity_prelude::{Context, Ready};

use crate::util::logger::Logger;

pub fn handle(_context: Context, ready: Ready) {
    Logger::notify(&format!("{} is online", ready.user.tag()));
}
