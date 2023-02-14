use url::Url;

pub async fn parse_url(url: &str) -> Result<String, ()> {
    if let Ok(url) = Url::parse(url) {
        match url.host_str() {
            Some("7tv.app") => parse_7tv(&url).await,
            Some("cdn.7tv.app") => parse_7tv_cdn(&url).await,
            Some("betterttv.com") => parse_bttv(&url).await,
            Some("cdn.betterttv.net") => parse_bttv_cdn(&url).await,
            Some("www.frankerfacez.com") => parse_ffz(&url),
            Some("cdn.frankerfacez.com") => parse_ffz_cdn(&url),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

async fn parse_7tv(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emotes", id] => try_ext(&format!("https://cdn.7tv.app/emote/{id}/2x")).await,
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

async fn parse_7tv_cdn(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emote", id, "1x.webp"]
            | ["emote", id, "2x.webp"]
            | ["emote", id, "3x.webp"]
            | ["emote", id, "4x.webp"] => {
                try_ext(&format!("https://cdn.7tv.app/emote/{id}/2x")).await
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

async fn parse_bttv(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emotes", id] => try_ext(&format!("https://cdn.betterttv.net/emote/{id}/2x")).await,
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

async fn parse_bttv_cdn(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emote", id, "1x.webp"]
            | ["emote", id, "2x.webp"]
            | ["emote", id, "3x.webp"]
            | ["emote", id, "4x.webp"] => {
                try_ext(&format!("https://cdn.betterttv.net/emote/{id}/2x")).await
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn parse_ffz(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emoticon", id_and_name] => {
                if let Some(id) = id_and_name.split('-').next() {
                    Ok(format!("https://cdn.frankerfacez.com/emoticon/{id}/2"))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn parse_ffz_cdn(url: &Url) -> Result<String, ()> {
    if let Some(segments) = url.path_segments() {
        match segments.collect::<Vec<_>>()[..] {
            ["emoticon", id, "1"]
            | ["emoticon", id, "2"]
            | ["emoticon", id, "3"]
            | ["emoticon", id, "4"] => Ok(format!("https://cdn.frankerfacez.com/emoticon/{id}/2")),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

async fn try_ext(url_without_ext: &str) -> Result<String, ()> {
    let gif_url = format!("{url_without_ext}.gif");
    let res = reqwest::get(&gif_url).await.or(Err(()))?;
    if res.status().is_success() {
        Ok(gif_url)
    } else {
        Ok(format!("{url_without_ext}.png"))
    }
}
