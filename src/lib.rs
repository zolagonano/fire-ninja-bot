use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use worker::*;

// TODO: Read Sources from config file
const MTPROTO_SOURCES: &[&str] = &["https://t.me/s/NextGenProxy", "https://t.me/s/MTP_roto"];

const HELP_MESSAGE: &str = "Fire Ninja Bot allows you to access proxies to bypass firewalls and access blocked content. Currently, only the following commands are available:

- /help: Shows this message.
- /mtproxy: Fetches and provides a list of MTProto proxies.
";

async fn fetch_source(source: &str) -> core::result::Result<String, &'static str> {
    let response = reqwest::get(source)
        .await
        .map_err(|_e| "Failed to fetch the source")?
        .text()
        .await
        .map_err(|_e| "Failed to load the content")?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
struct TelegramUpdate {
    message: Option<TelegramMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TelegramMessage {
    chat: TelegramChat,
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TelegramChat {
    id: i64,
}

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let update: Result<TelegramUpdate> = req.json().await;

    let update = match update {
        Ok(update) => update,
        Err(_) => return Response::error("Error parsing JSON", 400),
    };

    if let Some(message) = update.message {
        let chat_id = message.chat.id;
        let text = message.text.to_lowercase();

        let response_text = match text.as_str() {
            "/mtproxy" => {
                // NOTE: HashSet is used here to deduplicate the proxy links
                let mut proxy_list = HashSet::new();

                for source in MTPROTO_SOURCES {
                    if let Ok(raw_proxies) = fetch_source(source).await {
                        let proxies = proxy_scraper::Scraper::scrape_mtproxy(&raw_proxies);
                        for proxy in proxies {
                            proxy_list.insert(format!(
                                "https://t.me/proxy?server={}&port={}&secret={}",
                                proxy.host, proxy.port, proxy.secret
                            ));
                        }
                    }
                }
                proxy_list.into_iter().collect::<Vec<_>>().join("\n***\n")
            }
            "/help" => HELP_MESSAGE.to_string(),
            _ => "Invalid command, use /help to get list of available commands.".to_string(),
        };

        let response = json!({
            "method": "sendMessage",
            "chat_id": chat_id,
            "text": response_text,
        });

        return Response::from_json(&response);
    }

    Response::empty()
}
