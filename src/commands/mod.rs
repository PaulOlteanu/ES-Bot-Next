use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::application::command;
use twilight_model::application::interaction::{Interaction, InteractionData};
use worker::{Env, Response};

mod emote;

pub use emote::Emote;
use emote::EmoteAutocomplete;

pub mod util;
// pub use util;

pub fn command_specs() -> Vec<command::Command> {
    vec![Emote::create_command().into()]
}

// TODO: Match on command name
pub async fn run(interaction: Interaction, env: &Env) -> worker::Result<Response> {
    if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        if let Ok(command) = Emote::from_interaction(CommandInputData::from(*data)) {
            return command.run(env).await;
        }
    }

    Response::error("invalid args", 401)
}

pub async fn autocomplete(interaction: Interaction, env: &Env) -> worker::Result<Response> {
    if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        if let Ok(command) = EmoteAutocomplete::from_interaction(CommandInputData::from(*data)) {
            return command.run(env).await;
        }
    }

    Response::error("invalid args", 401)
}
