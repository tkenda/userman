use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use log::error;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::dao::{Dao, Memory};
use crate::{serialize_option_oid_as_string, serialize_vec_oid_as_string, Result};

pub const ADMIN_USERNAME: &str = "admin";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDB {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
    )]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub description: String,
    pub department: String,
    pub roles: Vec<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_oid_as_string"
    )]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub description: String,
    pub department: String,
    #[serde(serialize_with = "serialize_vec_oid_as_string")]
    pub roles: Vec<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            username: ADMIN_USERNAME.to_string(),
            password: Some("admin".to_string()),
            email: "admin@proteus.com.ar".to_string(),
            name: "admin".to_string(),
            surname: "admin".to_string(),
            description: Default::default(),
            department: "Super Admin".to_string(),
            roles: vec![],
            avatar: None,
            enabled: true,
            created_at: None,
            updated_at: None,
        }
    }
}

impl User {
    pub fn id(&self) -> ObjectId {
        self.id.unwrap_or_default()
    }

    pub fn verify<P: AsRef<[u8]>>(&self, password: P) -> bool {
        match self.password {
            Some(ref t) => match verify(password, t) {
                Ok(t) => t,
                Err(err) => {
                    error!("{}", err);
                    false
                }
            },
            None => false,
        }
    }

    pub fn hash_password(self) -> Self {
        match self.password {
            Some(t) => Self {
                password: Some(hash(t, DEFAULT_COST).unwrap()),
                ..self
            },
            None => self,
        }
    }

    pub fn hide_password(self) -> Self {
        Self {
            password: None,
            ..self
        }
    }

    pub fn into_create_db(self) -> UserDB {
        UserDB {
            id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            name: self.name,
            surname: self.surname,
            description: self.description,
            department: self.department,
            roles: self.roles,
            avatar: self.avatar,
            enabled: self.enabled,
            created_at: Some(DateTime::now()),
            updated_at: None,
        }
    }

    pub fn into_update_db(self) -> UserDB {
        UserDB {
            id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            name: self.name,
            surname: self.surname,
            description: self.description,
            department: self.department,
            roles: self.roles,
            avatar: self.avatar,
            enabled: self.enabled,
            created_at: None,
            updated_at: Some(DateTime::now())
        }
    }
}

#[derive(Clone)]
pub struct Users {
    users: Arc<RwLock<HashMap<ObjectId, User>>>,
    users_by_username: Arc<RwLock<HashMap<String, User>>>,
}

#[async_trait]
impl Memory<User> for Users {
    async fn load(dao: &Dao) -> Result<Self> {
        let (users, users_by_username) = dao.read_all_users().await?;

        Ok(Users {
            users: Arc::new(RwLock::new(users)),
            users_by_username: Arc::new(RwLock::new(users_by_username)),
        })
    }

    async fn reload(&self, dao: &Dao) -> Result<()> {
        let (users, users_by_username) = dao.read_all_users().await?;

        {
            let mut value = self.users.write().await;
            *value = users;
        }

        let mut value = self.users_by_username.write().await;
        *value = users_by_username;

        Ok(())
    }

    async fn get(&self, username: &String) -> Option<User> {
        self.users_by_username.read().await.get(username).cloned()
    }

    async fn get_by_id(&self, id: &ObjectId) -> Option<User> {
        self.users.read().await.get(id).cloned()
    }

    async fn get_all(&self) -> Vec<User> {
        self.users.read().await.clone().into_values().collect()
    }
}
