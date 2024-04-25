use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use worker::*;

// TODO: Read Sources from config file
const MTPROTO_SOURCES: &[&str] = &["https://t.me/s/NextGenProxy", "https://t.me/s/MTP_roto"];
const SHADOWSOCKS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

const VMESS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

const HELP_MESSAGE: &str = "Fire Ninja Bot allows you to access proxies to bypass firewalls and access blocked content. Currently, only the following commands are available:

- /help: Shows this message.
- /mtproxy: Fetches and provides a list of MTProto proxies.
- /shadowsocks: Fetches and provides a list of Shadowsocks proxies.
- /vmess: Fetches and provides a list of VMess proxies.
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
                                "[Host: {} Port: {}]({})",
                                proxy.host,
                                proxy.port,
                                proxy.to_url()
                            ));
                        }
                    }
                }
                proxy_list
                    .into_iter()
                    .take(15)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            "/ss" | "/shadowsocks" => {
                let mut proxy_list = HashSet::new();

                for source in SHADOWSOCKS_SOURCES {
                    if let Ok(raw_proxies) = fetch_source(source).await {
                        let proxies = proxy_scraper::Scraper::scrape_shadowsocks(&raw_proxies);
                        for proxy in proxies {
                            proxy_list.insert(format!("`{}`", proxy.to_url()));
                        }
                    }
                }
                proxy_list
                    .into_iter()
                    .take(15)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            "/vmess" => {
                let mut proxy_list = HashSet::new();

                for source in VMESS_SOURCES {
                    if let Ok(raw_proxies) = fetch_source(source).await {
                        let proxies = proxy_scraper::Scraper::scrape_vmess(&raw_proxies);
                        for proxy in proxies {
                            proxy_list.insert(format!("`{}`", proxy.to_url()));
                        }
                    }
                }
                proxy_list
                    .into_iter()
                    .take(10)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            "/help" | "/start" => HELP_MESSAGE.to_string(),
            _ => "Invalid command, use /help to get list of available commands.".to_string(),
        };

        let response = json!({
            "method": "sendMessage",
            "chat_id": chat_id,
            "text": response_text,
            "parse_mode": "Markdown",
        });

        return Response::from_json(&response);
    }

    Response::empty()
}
