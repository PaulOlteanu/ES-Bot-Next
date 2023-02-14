use twilight_interactions::command::{CommandModel, CreateCommand};
use worker::kv::KvStore;
use worker::Response;

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
#[command(name = "emote", desc = "Add or Send emotes", autocomplete = true)]
pub enum EmoteAutocomplete {
    #[command(name = "send")]
    Send(SendAutocomplete),
}

impl Emote {
    pub async fn run(&self, emote_store: &KvStore) -> worker::Result<Response> {
        match self {
            Self::Add(add) => add.run(emote_store).await,
            Self::Send(send) => send.run(emote_store).await,
        }
    }
}

impl EmoteAutocomplete {
    pub async fn run(&self, emote_store: &KvStore) -> worker::Result<Response> {
        match self {
            Self::Send(send) => send.run(emote_store).await,
        }
    }
}
