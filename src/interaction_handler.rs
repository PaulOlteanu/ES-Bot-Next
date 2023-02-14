use twilight_model::application::interaction::{Interaction, InteractionType};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use worker::kv::KvStore;
use worker::{console_log, Response, Result};

use crate::commands;

pub async fn handle(interaction: Interaction, emote_store: &KvStore) -> Result<Response> {
    match interaction.kind {
        InteractionType::Ping => {
            let response = InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            };

            Response::from_json(&response)
        }

        InteractionType::ApplicationCommand => {
            console_log!("Received command");
            commands::run(interaction, emote_store).await
        }

        InteractionType::ApplicationCommandAutocomplete => {
            console_log!("Received autocomplete request");

            commands::autocomplete(interaction, emote_store).await

            // Response::error("Unhandled interaction type", 400)
        }

        _ => {
            console_log!("Unhandled interaction type");
            Response::error("Unhandled interaction type", 400)
        }
    }
}
