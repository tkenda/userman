use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use userman_auth::role::RoleItems;
use userman_auth::app::App;

use crate::dao::{Dao, Memory};
use crate::Result;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppDB {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub version: u64,
    pub default_role: RoleItems,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

impl From<&App> for AppDB {
    fn from(src: &App) -> Self {
        if src.id.is_none() {
            Self {
                id: None,
                name: src.name.clone(),
                version: src.version,
                default_role: src.default_role.clone(),
                created_at: Some(DateTime::now()),
                updated_at: None,
            }
        } else {
            Self {
                id: src.id,
                name: src.name.clone(),
                version: src.version,
                default_role: src.default_role.clone(),
                created_at: None,
                updated_at: Some(DateTime::now()),
            }
        }
    }
}

#[derive(Clone)]
pub struct Apps(Arc<RwLock<HashMap<ObjectId, App>>>);

#[async_trait]
impl Memory<App> for Apps {
    async fn load(dao: &Dao) -> Result<Self> {
        let apps = dao.read_all_apps().await?;

        Ok(Self(Arc::new(RwLock::new(apps))))
    }

    async fn reload(&self, dao: &Dao) -> Result<()> {
        let mut value = self.0.write().await;
        *value = dao.read_all_apps().await?;
        Ok(())
    }

    async fn get_by_id(&self, id: &ObjectId) -> Option<App> {
        self.0.read().await.get(id).cloned()
    }

    async fn get_all(&self) -> Vec<App> {
        self.0.read().await.clone().into_values().collect()
    }
}
