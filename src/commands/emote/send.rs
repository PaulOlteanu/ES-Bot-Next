use twilight_interactions::command::{AutocompleteValue, CommandModel, CreateCommand};
use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceValue};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::{Env, Response};

use crate::commands::util;

#[derive(Debug, CommandModel, CreateCommand)]
#[command(name = "send", desc = "Send an emote")]
pub struct SendEmote {
    /// Emote name
    #[command(autocomplete = true)]
    emote_name: String,
}

#[derive(Debug, CommandModel)]
#[command(autocomplete = true)]
pub struct SendAutocomplete {
    emote_name: AutocompleteValue<String>,
}

impl SendEmote {
    pub async fn run(&self, env: &Env) -> worker::Result<Response> {
        let emote_store = env.kv("emote_store")?;
        if let Some(url) = emote_store.get(&self.emote_name).text().await? {
            let response_data = InteractionResponseDataBuilder::new().content(url).build();

            let response = InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response_data),
            };

            Response::from_json(&response)
        } else {
            // TODO: This should respond to the user
            Response::error("unknown url", 400)
        }
    }
}

impl SendAutocomplete {
    pub async fn run(&self, env: &Env) -> worker::Result<Response> {
        let emote_store = env.kv("emote_store")?;
        // TODO: Handle cursor
        let emotes = emote_store.list().execute().await?;

        if let AutocompleteValue::Focused(ref emote_name) = self.emote_name {
            if emote_name.is_empty() {
                return util::autocomplete_from_choices([]);
            }

            let choices: Vec<_> = emotes
                .keys
                .iter()
                .filter(|k| {
                    k.name
                        .to_ascii_lowercase()
                        .starts_with(&emote_name.to_ascii_lowercase())
                })
                .map(|k| CommandOptionChoice {
                    name: k.name.clone(),
                    value: CommandOptionChoiceValue::String(k.name.clone()),
                    name_localizations: None,
                })
                .take(25)
                .collect();

            util::autocomplete_from_choices(choices)
        } else {
            Response::error("unexpected autocomplete field", 500)
        }
    }
}
