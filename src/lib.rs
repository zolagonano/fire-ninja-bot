mod config;
mod db;
mod telegram;
mod utils;

use crate::config::*;
use crate::db::*;
use crate::telegram::*;
use crate::utils::*;
use proxy_scraper::*;
use std::collections::HashSet;
use worker::*;

#[derive(Debug)]
enum Command {
    MTProxy,
    Shadowsocks,
    VMess,
    VLess,
    Trojan,
    Hysteria,
    TUIC,
    Help,
    Start,
    Subscribe(Vec<Command>),
    Unsubscribe,
    Support,
}

impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        let cmd = command.to_lowercase();
        match cmd.as_str() {
            cmd if cmd.starts_with(SUBSCRIBE_COMMAND) => {
                Some(Self::Subscribe(Self::containing_proxies(&cmd)))
            }
            MTPROXY_COMMAND => Some(Self::MTProxy),
            SHADOWSOCKS_COMMAND => Some(Self::Shadowsocks),
            VMESS_COMMAND => Some(Self::VMess),
            VLESS_COMMAND => Some(Self::VLess),
            TROJAN_COMMAND => Some(Self::Trojan),
            HYSTERIA_COMMAND => Some(Self::Hysteria),
            TUIC_COMMAND => Some(Self::TUIC),
            HELP_COMMAND => Some(Self::Help),
            START_COMMAND => Some(Self::Start),
            UNSUBSCRIBE_COMMAND => Some(Self::Unsubscribe),

            SUPPORT_COMMAND => Some(Self::Support),
            _ => None,
        }
    }

    pub fn containing_proxies(command: &str) -> Vec<Command> {
        let mut proxy_commands: Vec<Command> = Vec::new();

        match command {
            _ if command.contains("mtproxy") => proxy_commands.push(Command::MTProxy),
            _ if command.contains("shadowsocks") => proxy_commands.push(Command::Shadowsocks),
            _ if command.contains("vmess") => proxy_commands.push(Command::VMess),
            _ if command.contains("vless") => proxy_commands.push(Command::VLess),
            _ if command.contains("trojan") => proxy_commands.push(Command::Trojan),
            _ if command.contains("hysteria") => proxy_commands.push(Command::Hysteria),
            _ if command.contains("tuic") => proxy_commands.push(Command::TUIC),
            _ => {}
        };

        proxy_commands
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
            Command::Start => HELP_MESSAGE.to_string(),
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
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let update: Result<TelegramUpdate> = req.json().await;

    let update = match update {
        Ok(update) => update,
        Err(_) => return Response::error("Error parsing JSON", 400),
    };

    if let Some(message) = update.message {
        let chat_id = message.chat.id;
        let text = message.text.to_lowercase();

        let db = DB::from_env(env)?;
        let response_text = match Command::from_str(text.as_str()) {
            command @ Some(Command::Start) => {
                db.add_new_user(chat_id).await?;

                command.unwrap().fetch_and_scrape().await
            }

            Some(Command::Subscribe(proxy_types)) => {
                if proxy_types.is_empty() {
                    SUBSCRIPTION_FAILED_MESSAGE.to_string()
                } else {
                    db.add_new_subscribed_user(chat_id, proxy_types).await?;
                    SUBSCRIPTION_MESSAGE.to_string()
                }
            }
            Some(Command::Unsubscribe) => {
                db.del_subscribed_user(chat_id).await?;
                UNSUBSCRIBE_MESSAGE.to_string()
            }
            Some(command) => command.fetch_and_scrape().await,
            None => INVALID_COMMAND_MESSAGE.to_string(),
        };

        let response = TelegramSendMessage::new_md(chat_id, response_text).to_json();

        return Response::from_json(&response);
    }

    Response::empty()
}

#[event(scheduled)]
async fn main(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) -> () {
    let db = DB::from_env(env).unwrap();

    let subscribed_users = db.fetch_subscribed_users().await.unwrap();

    for user in subscribed_users {
        let subscribed_command = format!("{} {:?}", SUBSCRIBE_COMMAND, user.proxy_types);
        let subscribed_proxies = Command::from_str(&subscribed_command);

        if let Some(Command::Subscribe(sub_proxies)) = subscribed_proxies {
            for sub_proxy in sub_proxies {
                let mut response_text = String::new();
                response_text.push_str(&format!("SUBSCRIBED {:?} PROXIES:\n\n", sub_proxy));
                response_text.push_str(&sub_proxy.fetch_and_scrape().await);

                TelegramSendMessage::new_md(user.user_id as i64, response_text)
                    .send_directly(BOT_TOKEN)
                    .await;
            }
        }
    }
}
