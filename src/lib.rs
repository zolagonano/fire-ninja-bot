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

const TROJAN_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

const HELP_MESSAGE: &str = "Fire Ninja Bot allows you to access proxies to bypass firewalls and access blocked content. Currently, only the following commands are available:

- /help: Shows this message.
- /mtproxy: Fetches and provides a list of MTProto proxies.
- /shadowsocks: Fetches and provides a list of Shadowsocks proxies.
- /vmess: Fetches and provides a list of VMess proxies.
- /vless: Fetches and provides a list of VLess proxies.
- /trojan: Fetches and provides a list of Trojan proxies.
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
    Trojan,
    Help,
}

impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "/mtproxy" => Some(Self::MTProxy),
            "/ss" | "/shadowsocks" => Some(Self::Shadowsocks),
            "/vmess" => Some(Self::VMess),
            "/vless" => Some(Self::VLess),
            "/trojan" => Some(Self::Trojan),
            "/start" | "/help" => Some(Self::Help),
            _ => None,
        }
    }

    pub fn get_sources(&self) -> &[&str] {
        match self {
            Command::MTProxy => MTPROTO_SOURCES,
            Command::Shadowsocks => SHADOWSOCKS_SOURCES,
            Command::VMess => VMESS_SOURCES,
            Command::VLess => VLESS_SOURCES,
            Command::Trojan => TROJAN_SOURCES,
            _ => &[""],
        }
    }

    pub async fn fetch_and_scrape(&self) -> String {
        let mut proxy_list = HashSet::new();
        let raw_proxies = fetch_sources(self.get_sources()).await;

        match self {
            Command::MTProxy => proxy_scraper::Scraper::scrape_mtproxy(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("[{}]({})", proxy.to_url(), proxy.to_url()));
                }),
            Command::Shadowsocks => proxy_scraper::Scraper::scrape_shadowsocks(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }),
            Command::VMess => proxy_scraper::Scraper::scrape_vmess(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }),
            Command::VLess => proxy_scraper::Scraper::scrape_vless(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }),
            Command::Trojan => proxy_scraper::Scraper::scrape_trojan(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url()));
                }),
            _ => (),
        };

        match self {
            Command::Help => HELP_MESSAGE.to_string(),
            _ => proxy_list
                .into_iter()
                .take(8)
                .collect::<Vec<_>>()
                .join("\n﹌﹌﹌\n"),
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
            Some(command) => command.fetch_and_scrape().await,
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
