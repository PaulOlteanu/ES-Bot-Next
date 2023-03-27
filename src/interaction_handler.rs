use twilight_model::application::interaction::{Interaction, InteractionType};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use worker::{Env, Response};

use crate::commands;

pub async fn handle(interaction: Interaction, env: &Env) -> worker::Result<Response> {
    match interaction.kind {
        InteractionType::Ping => {
            let response = InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            };

            Response::from_json(&response)
        }

        InteractionType::ApplicationCommand => commands::run(interaction, env).await,

        InteractionType::ApplicationCommandAutocomplete => {
            commands::autocomplete(interaction, env).await
        }

        _ => Response::error("unhandled interaction type", 400),
    }
}
