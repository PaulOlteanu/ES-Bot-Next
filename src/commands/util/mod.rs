use twilight_model::application::command::CommandOptionChoice;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::Response;

pub fn autocomplete_from_choices(
    choices: impl IntoIterator<Item = CommandOptionChoice>,
) -> worker::Result<Response> {
    let response_data = InteractionResponseDataBuilder::new()
        .choices(choices)
        .build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(response_data),
    };

    Response::from_json(&response)
}
