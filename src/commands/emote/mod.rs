use twilight_interactions::command::{CommandModel, CreateCommand};
use worker::{Env, Response};

mod add;
mod parser;
mod send;

use add::AddEmote;
use send::{SendAutocomplete, SendEmote};

#[derive(Debug, CommandModel, CreateCommand)]
#[command(name = "emote", desc = "Add or Send emotes")]
pub enum Emote {
    #[command(name = "add")]
    Add(AddEmote),
    #[command(name = "send")]
    Send(SendEmote),
}

#[derive(Debug, CommandModel)]
#[command(autocomplete = true)]
pub enum EmoteAutocomplete {
    #[command(name = "send")]
    Send(SendAutocomplete),
}

impl Emote {
    pub async fn run(&self, env: &Env) -> worker::Result<Response> {
        match self {
            Self::Add(add) => add.run(env).await,
            Self::Send(send) => send.run(env).await,
        }
    }
}

impl EmoteAutocomplete {
    pub async fn run(&self, env: &Env) -> worker::Result<Response> {
        match self {
            Self::Send(send) => send.run(env).await,
        }
    }
}
