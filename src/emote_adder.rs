use serde::{Deserialize, Serialize};
use worker::{Env, Request, Response};

#[derive(Serialize, Deserialize)]
struct ReqStruct {
    name: String,
    url: String,
}

pub async fn handle(mut req: Request, env: Env) -> worker::Result<Response> {
    let body = req.text().await?;
    let r: ReqStruct = serde_json::from_str(&body)?;

    if let Ok(url) = crate::commands::emote::parser::parse_url(&r.url).await {
        let emote_store = env.kv("emote_store")?;
        emote_store.put(&r.name, url)?.execute().await?;
        Response::ok("added")
    } else {
        Response::error("didn't add", 400)
    }
}
