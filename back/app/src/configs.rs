use async_trait::async_trait;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::dao::{Dao, Memory};
use crate::{Result, UsermanError};

pub static TOKEN_CONFIG: &str = "token";

trait ConfigProps {
    fn label() -> &'static str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenConfig {
    pub secret: String,
    pub duration: i64,
}

impl ConfigProps for TokenConfig {
    fn label() -> &'static str {
        TOKEN_CONFIG
    }
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            secret: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
            duration: 3600 * 4,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ConfigData {
    Token(TokenConfig),
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "_id")]
    pub id: String,
    pub data: ConfigData,
}

impl Config {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn new_token() -> Self {
        Self {
            id: TokenConfig::label().to_string(),
            data: ConfigData::Token(TokenConfig::default()),
        }
    }

    pub fn unwrap_token(&self) -> Result<TokenConfig> {
        match self.data {
            ConfigData::Token(ref t) => Ok(t.clone()),
            #[allow(unreachable_patterns)]
            _ => Err(UsermanError::GetConfig(TOKEN_CONFIG)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Configs(Arc<RwLock<HashMap<String, ConfigData>>>);

#[async_trait]
impl Memory<ConfigData, String> for Configs {
    async fn load(dao: &Dao) -> Result<Self> {
        let configs = dao.read_all_configs().await?;

        Ok(Self(Arc::new(RwLock::new(configs))))
    }

    async fn reload(&self, dao: &Dao) -> Result<()> {
        let mut value = self.0.write().await;
        *value = dao.read_all_configs().await?;
        Ok(())
    }

    async fn get(&self, id: &String) -> Option<ConfigData> {
        self.0.read().await.get(id).cloned()
    }
}
