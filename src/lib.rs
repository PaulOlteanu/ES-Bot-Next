use ed25519_dalek::{PublicKey, Signature, Verifier};
use twilight_model::application::interaction::Interaction;
use worker::*;

pub mod commands;
mod interaction_handler;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(mut req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    if req.path() != "/" || req.method() != Method::Post {
        return Response::error("Invalid request", 400);
    }

    let headers = req.headers();
    let timestamp = if let Some(ts) = headers.get("x-signature-timestamp")? {
        ts
    } else {
        return Response::error("Bad Request", 400);
    };

    let signature = if let Some(sig) = headers.get("x-signature-ed25519")? {
        sig
    } else {
        return Response::error("Bad Request", 400);
    };

    let signature = hex::decode(&signature).unwrap();
    let signature = Signature::from_bytes(&signature).unwrap();

    let discord_public_key = env.var("DISCORD_PUBLIC_KEY")?.to_string();
    let public_key = hex::decode(&discord_public_key).unwrap();
    let public_key = PublicKey::from_bytes(&public_key).unwrap();

    let body = req.text().await?;

    if public_key
        .verify(format!("{}{}", timestamp, body).as_bytes(), &signature)
        .is_err()
    {
        return Response::error("Verification Failed", 401);
    }

    let emote_store = env.kv("emote_store")?;

    let interaction: Interaction = serde_json::from_str(&body)?;
    interaction_handler::handle(interaction, &emote_store).await
}
