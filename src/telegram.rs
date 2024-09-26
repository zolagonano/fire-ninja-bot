use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramUpdate {
    pub message: Option<TelegramMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramMessage {
    pub chat: TelegramChat,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramChat {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramSendMessage {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: String,
}

impl TelegramSendMessage {
    pub fn new_md(chat_id: i64, text: String) -> Self {
        TelegramSendMessage {
            chat_id,
            text,
            parse_mode: "Markdown".to_string(),
        }
    }

    pub fn to_json(&self) -> JsonValue {
        json!({
            "method": "sendMessage",
            "chat_id": self.chat_id,
            "text": self.text,
            "parse_mode": self.parse_mode,
        })
    }

    pub async fn send_directly(&self, token: &str) -> () {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

        let client = reqwest::Client::new();
        let _response = client.post(&url).json(&self).send().await.unwrap();
    }
}
