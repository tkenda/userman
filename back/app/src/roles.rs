use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use userman_auth::roles::{Role, RoleItems};

use crate::dao::{Dao, Memory};
use crate::serialize_option_oid_as_string;
use crate::Result;

/* Role */

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoleDB {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub app: ObjectId,
    pub name: String,
    pub items: RoleItems,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

impl From<&Role> for RoleDB {
    fn from(role: &Role) -> Self {
        if role.id.is_none() {
            Self {
                id: None,
                app: role.app,
                name: role.name.clone(),
                items: role.items.clone(),
                created_at: Some(DateTime::now()),
                updated_at: None,
            }
        } else {
            RoleDB {
                id: role.id,
                app: role.app,
                name: role.name.clone(),
                items: role.items.clone(),
                created_at: None,
                updated_at: Some(DateTime::now()),
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoleName {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_oid_as_string"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
}

impl From<&Role> for RoleName {
    fn from(role: &Role) -> Self {
        Self {
            id: role.id,
            name: role.name.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Roles {
    roles: Arc<RwLock<HashMap<ObjectId, Role>>>,
    roles_by_name: Arc<RwLock<HashMap<String, Role>>>,
}

#[async_trait]
impl Memory<Role> for Roles {
    async fn load(dao: &Dao) -> Result<Self> {
        let (roles, roles_by_name) = dao.read_all_roles().await?;

        Ok(Self {
            roles: Arc::new(RwLock::new(roles)),
            roles_by_name: Arc::new(RwLock::new(roles_by_name)),
        })
    }

    async fn reload(&self, dao: &Dao) -> Result<()> {
        let (roles, roles_by_name) = dao.read_all_roles().await?;

        {
            let mut value = self.roles.write().await;
            *value = roles;
        }

        let mut value = self.roles_by_name.write().await;
        *value = roles_by_name;

        Ok(())
    }

    async fn get_by_id(&self, id: &ObjectId) -> Option<Role> {
        self.roles.read().await.get(id).cloned()
    }

    async fn get(&self, name: &String) -> Option<Role> {
        self.roles_by_name.read().await.get(name).cloned()
    }

    async fn get_all(&self) -> Vec<Role> {
        self.roles.read().await.clone().into_values().collect()
    }
}
