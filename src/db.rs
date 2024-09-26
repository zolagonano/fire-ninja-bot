use crate::config::*;
use crate::Command;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedUser {
    pub user_id: f64,
    pub proxy_types: String,
}

pub struct DB {
    d1: D1Database,
}

impl DB {
    pub fn from_env(env: Env) -> Result<Self> {
        let d1 = env.d1(D1_DB_NAME)?;
        Ok(DB::new(d1))
    }

    pub fn new(d1: D1Database) -> Self {
        DB { d1 }
    }
    pub async fn add_new_user(&self, user_id: i64) -> Result<()> {
        let query = self.d1.prepare(format!(
            "INSERT into users (id) VALUES ({user_id}) ON CONFLICT (id) DO NOTHING"
        ));
        let result = query.run().await;

        match result {
            Ok(_) => Ok(()),
            Err(_e) => Err(FAILED_TO_ADD_USER_MESSAGE.into()),
        }
    }

    pub async fn add_new_subscribed_user(
        &self,
        user_id: i64,
        subscribed_proxies: Vec<Command>,
    ) -> Result<()> {
        let proxy_types = format!("\"{:?}\"", subscribed_proxies).to_lowercase();
        let query = self.d1.prepare(format!(
                        "INSERT into subscribed_users (user_id, proxy_types) VALUES ({user_id}, {proxy_types}) ON CONFLICT (user_id) DO UPDATE SET proxy_types = {proxy_types}"));

        let result = query.run().await;

        match result {
            Ok(_) => Ok(()),
            Err(_e) => Err(FAILED_TO_ADD_USER_MESSAGE.into()),
        }
    }

    pub async fn fetch_subscribed_users(&self) -> Result<Vec<SubscribedUser>> {
        let statement = self.d1.prepare("SELECT * FROM subscribed_users");
        let result = statement.all().await?;

        let result = result.results::<SubscribedUser>()?;

        Ok(result)
    }

    pub async fn del_subscribed_user(&self, user_id: i64) -> Result<()> {
        let query = self.d1.prepare(format!(
            "DELETE FROM subscribed_users WHERE user_id = {user_id};"
        ));
        let result = query.run().await;

        match result {
            Ok(_) => Ok(()),
            Err(_e) => Err(FAILED_TO_DELETE_USER_MESSAGE.into()),
        }
    }
}
