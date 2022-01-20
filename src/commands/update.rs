// Plagiarized from https://github.com/kangalioo/poise/blob/master/src/samples.rs

use poise::{command, say_reply, serenity_prelude::CreateApplicationCommands};

use crate::{
    types::poise::{Context, Error},
    util::logger::Logger,
};

/// Update the list of available commands.
#[command(
    rename = "update",
    slash_command,
    prefix_command,
    hide_in_help,
    owners_only
)]
pub async fn exec(
    context: Context<'_>,
    #[flag]
    #[description = "Whether to update the global list"]
    global: bool,
) -> Result<(), Error> {
    register_commands(context, global).await?;

    let scope = if global {
        "globally"
    } else {
        "inside this server"
    };

    say_reply(
        context,
        format!("Successfully updated available commands {scope}"),
    )
    .await?;

    Ok(())
}

async fn register_commands(context: Context<'_>, global: bool) -> Result<(), Error> {
    Logger::debug("Checking if invoked inside guild");
    let guild = match context.guild() {
        Some(x) => x,
        None => {
            say_reply(context, "Update only possible in a server").await?;
            return Ok(());
        }
    };

    Logger::debug("Gathering commands");
    let commands = &context.framework().options().application_options.commands;
    let mut commands_builder = CreateApplicationCommands::default();

    for cmd in commands {
        commands_builder.create_application_command(|f| cmd.create(f));
    }

    let json_value = poise::serde_json::Value::Array(commands_builder.0);

    if global {
        Logger::debug("Updating global command list");
        context
            .discord()
            .http
            .create_global_application_commands(&json_value)
            .await?;
    } else {
        Logger::debug("Updating guild command list");
        context
            .discord()
            .http
            .create_guild_application_commands(guild.id.0, &json_value)
            .await?;
    }

    Ok(())
}
