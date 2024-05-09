mod config;
mod telegram;
mod utils;

use crate::config::*;
use crate::telegram::*;
use crate::utils::*;
use proxy_scraper::*;
use std::collections::HashSet;
use worker::*;

enum Command {
    MTProxy,
    Shadowsocks,
    VMess,
    VLess,
    Trojan,
    Hysteria,
    TUIC,
    Help,
    Support,
}

impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            MTPROXY_COMMAND => Some(Self::MTProxy),
            SHADOWSOCKS_COMMAND => Some(Self::Shadowsocks),
            VMESS_COMMAND => Some(Self::VMess),
            VLESS_COMMAND => Some(Self::VLess),
            TROJAN_COMMAND => Some(Self::Trojan),
            HYSTERIA_COMMAND => Some(Self::Hysteria),
            TUIC_COMMAND => Some(Self::TUIC),
            START_COMMAND | HELP_COMMAND => Some(Self::Help),
            SUPPORT_COMMAND => Some(Self::Support),
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
            Command::Hysteria => HYSTERIA_SOURCES,
            Command::TUIC => TUIC_SOURCES,
            _ => &[""],
        }
    }

    pub async fn fetch_and_scrape(&self) -> String {
        let mut proxy_list = HashSet::new();
        let raw_proxies = fetch_sources(self.get_sources()).await;

        match self {
            Command::MTProxy => mtproxy::MTProxy::scrape(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("[{}]({})", proxy.to_url(), proxy.to_url()));
                }),
            Command::Shadowsocks => shadowsocks::Shadowsocks::scrape(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url_pretty()));
                }),
            Command::VMess => vmess::VMess::scrape(&raw_proxies).iter().for_each(|proxy| {
                proxy_list.insert(format!("`{}`", proxy.to_url()));
            }),
            Command::VLess => vless::VLess::scrape(&raw_proxies).iter().for_each(|proxy| {
                proxy_list.insert(format!("`{}`", proxy.to_url_pretty()));
            }),
            Command::Trojan => trojan::Trojan::scrape(&raw_proxies)
                .iter()
                .for_each(|proxy| {
                    proxy_list.insert(format!("`{}`", proxy.to_url_pretty()));
                }),
            Command::Hysteria => {
                hysteria::Hysteria::scrape(&raw_proxies)
                    .iter()
                    .for_each(|proxy| {
                        proxy_list.insert(format!("`{}`", proxy.to_url_pretty()));
                    })
            }
            Command::TUIC => tuic::TUIC::scrape(&raw_proxies).iter().for_each(|proxy| {
                proxy_list.insert(format!("`{}`", proxy.to_url()));
            }),
            _ => (),
        };

        let result = match self {
            Command::Help => HELP_MESSAGE.to_string(),
            Command::Support => SUPPORT_MESSAGE.to_string(),
            _ => proxy_list
                .into_iter()
                .take(8)
                .collect::<Vec<_>>()
                .join(LINE_SEPERATOR),
        };

        if result.is_empty() {
            return NO_PROXIES_FOUND_MESSAGE.to_string();
        }

        result
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
            None => INVALID_COMMAND_MESSAGE.to_string(),
        };

        let response = TelegramSendMessage::new_md(chat_id, response_text).to_json();

        return Response::from_json(&response);
    }

    Response::empty()
}
