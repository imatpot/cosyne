// Plagiarized from https://github.com/kangalioo/poise/blob/master/src/samples.rs

use poise::{say_reply, serenity_prelude::CreateApplicationCommands};

use crate::{
    types::poise::{Context, Error},
    util::logger::Logger,
};

#[poise::command(rename = "register", prefix_command, hide_in_help)]
pub async fn execute(context: Context<'_>, #[flag] _global: bool) -> Result<(), Error> {
    Logger::info(&format!(
        "{} ran [register] in {}",
        context.author().tag(),
        context.guild().unwrap().name
    ));

    if context.author().id == 324943524132814849 {
        register_commands(context, false).await?;
        say_reply(context, "All done").await?;
        Logger::info(&format!(
            "Successfully registered commands in {}",
            context.guild().unwrap().name
        ));
    } else {
        Logger::warn(&format!(
            "{} was rejected from registering slash commands",
            context.author().tag()
        ));
    }

    Ok(())
}

async fn register_commands(context: Context<'_>, global: bool) -> Result<(), Error> {
    let guild = match context.guild() {
        Some(x) => x,
        None => {
            say_reply(context, "Must be called in guild").await?;
            return Ok(());
        }
    };

    let mut commands_builder = CreateApplicationCommands::default();
    let commands = &context.framework().options().application_options.commands;
    for cmd in commands {
        commands_builder.create_application_command(|f| cmd.create(f));
    }

    let json_value = poise::serde_json::Value::Array(commands_builder.0);
    if global {
        context
            .discord()
            .http
            .create_global_application_commands(&json_value)
            .await?;
    } else {
        context
            .discord()
            .http
            .create_guild_application_commands(guild.id.0, &json_value)
            .await?;
    }

    Ok(())
}
