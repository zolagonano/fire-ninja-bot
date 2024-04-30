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

const VLESS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

const HELP_MESSAGE: &str = "Fire Ninja Bot allows you to access proxies to bypass firewalls and access blocked content. Currently, only the following commands are available:

- /help: Shows this message.
- /mtproxy: Fetches and provides a list of MTProto proxies.
- /shadowsocks: Fetches and provides a list of Shadowsocks proxies.
- /vmess: Fetches and provides a list of VMess proxies.
- /vless: Fetches and provides a list of VLess proxies.
";

async fn fetch_sources(sources: &[&str]) -> String {
    let mut content = String::new();
    for source in sources {
        if let Ok(response) = reqwest::get(*source).await {
            if let Ok(text) = response.text().await {
                content.push_str(&text);
            }
        }
    }

    content
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

enum Command {
    MTProxy,
    Shadowsocks,
    VMess,
    VLess,
    Help,
}

impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "/mtproxy" => Some(Self::MTProxy),
            "/ss" | "/shadowsocks" => Some(Self::Shadowsocks),
            "/vmess" => Some(Self::VMess),
            "/vless" => Some(Self::VLess),
            "/start" | "/help" => Some(Self::Help),
            _ => None,
        }
    }
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

        let response_text = match Command::from_str(text.as_str()) {
            Some(Command::MTProxy) => {
                // NOTE: HashSet is used here to deduplicate the proxy links
                let mut proxy_list = HashSet::new();
                let raw_proxies = fetch_sources(MTPROTO_SOURCES).await;
                let proxies = proxy_scraper::Scraper::scrape_mtproxy(&raw_proxies);
                for proxy in proxies {
                    proxy_list.insert(format!(
                        "[Host: {} Port: {}]({})",
                        proxy.host,
                        proxy.port,
                        proxy.to_url()
                    ));
                }

                proxy_list
                    .into_iter()
                    .take(15)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            Some(Command::Shadowsocks) => {
                let mut proxy_list = HashSet::new();

                let raw_proxies = fetch_sources(SHADOWSOCKS_SOURCES).await;
                let proxies = proxy_scraper::Scraper::scrape_shadowsocks(&raw_proxies);
                for proxy in proxies {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }

                proxy_list
                    .into_iter()
                    .take(12)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            Some(Command::VMess) => {
                let mut proxy_list = HashSet::new();

                let raw_proxies = fetch_sources(VMESS_SOURCES).await;
                let proxies = proxy_scraper::Scraper::scrape_vmess(&raw_proxies);
                for proxy in proxies {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }
                proxy_list
                    .into_iter()
                    .take(8)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            Some(Command::VLess) => {
                let mut proxy_list = HashSet::new();

                let raw_proxies = fetch_sources(VLESS_SOURCES).await;
                let proxies = proxy_scraper::Scraper::scrape_vless(&raw_proxies);
                for proxy in proxies {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }
                proxy_list
                    .into_iter()
                    .take(8)
                    .collect::<Vec<_>>()
                    .join("\n﹌﹌﹌\n")
            }
            Some(Command::Help) => HELP_MESSAGE.to_string(),
            None => "Invalid command, use /help to get list of available commands.".to_string(),
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
