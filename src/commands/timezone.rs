use poise::{command, say_reply};
use regex::Regex;

use crate::{
    types::poise::{Context, Error},
    util::logger::Logger,
};

/// Convert 24h format times across IANA timezones (e.g. Europe/Oslo).
#[command(rename = "timezone", slash_command, prefix_command)]
pub async fn exec(
    context: Context<'_>,
    #[description = "Time in 24h format"] time: String,
    #[description = "Source time zone in IANA format"] source_tz: String,
    #[description = "Target time zone in IANA format"] target_tz: String,
) -> Result<(), Error> {
    let is_24h_time = Regex::new(r"(?:[01]?|2(?![4-9]))\d:[0-5]\d").unwrap();

    if !is_24h_time.is_match(&time) {
        say_reply(
            context,
            &format!("Sorry, but `{time}` is not proper 24h format"),
        )
        .await?;
        return Ok(());
    }

    Ok(())
}
