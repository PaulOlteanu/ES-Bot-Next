use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::{Env, Response};

use crate::commands::emote::parser;

#[derive(Debug, CreateCommand, CommandModel)]
#[command(name = "add", desc = "Add an emote")]
pub struct AddEmote {
    /// Name for the emote
    #[command(rename = "name")]
    emote_name: String,

    /// Url for the emote
    url: String,
}

impl AddEmote {
    pub async fn run(&self, env: &Env) -> worker::Result<Response> {
        if let Ok(url) = parser::parse_url(&self.url).await {
            let response_data = InteractionResponseDataBuilder::new()
                .content(format!("adding emote {}", self.emote_name))
                .build();

            let emote_store = env.kv("emote_store")?;

            emote_store.put(&self.emote_name, url)?.execute().await?;

            let response = InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response_data),
            };

            Response::from_json(&response)
        } else {
            Response::error("invalid url", 400)
        }
    }
}
