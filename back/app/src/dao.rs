use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc};
use mongodb::options::FindOptions;
use mongodb::options::IndexOptions;
use mongodb::Database;
use mongodb::IndexModel;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

use crate::apps::AppDB;
use crate::configs::{Config, ConfigData, TOKEN_CONFIG};
use crate::roles::RoleDB;
use crate::tokens::{RefreshToken, SessionToken};
use crate::users::{User, ADMIN_USERNAME};
use crate::watchers::Event;
use crate::{Result, UmtError};

use userman_auth::app::{App, LOCAL_APP};
use userman_auth::role::{Role, LOCAL_ROLE};

const ROLES: &str = "roles";
const CONFIGS: &str = "configs";
const TOKENS: &str = "tokens";
const USERS: &str = "users";
const APPS: &str = "apps";

#[async_trait]
pub trait Memory<T, I = String> {
    async fn load(dao: &Dao) -> Result<Self>
    where
        Self: std::marker::Sized;
    async fn reload(&self, dao: &Dao) -> Result<()>;

    async fn get(&self, _id: &I) -> Option<T> {
        None
    }

    async fn get_by_id(&self, _id: &ObjectId) -> Option<T> {
        None
    }

    async fn get_all(&self) -> Vec<T> {
        vec![]
    }
}

#[derive(Clone)]
pub struct Dao {
    database: Database,
}

impl Dao {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /* CONFIGS */

    pub async fn create_config(&self, config: &Config) -> Result<Option<ObjectId>> {
        self.database
            .collection::<Config>(CONFIGS)
            .insert_one(config, None)
            .await
            .map(|t| t.inserted_id.as_object_id())
            .map_err(UmtError::MongoInsertOne)
    }

    pub async fn read_config(&self, id: &str) -> Result<Option<Config>> {
        self.database
            .collection::<Config>(CONFIGS)
            .find_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await
            .map_err(UmtError::MongoFindOne)
    }

    pub async fn read_all_configs(&self) -> Result<HashMap<String, ConfigData>> {
        let mut cursor = self
            .database
            .collection::<Config>(CONFIGS)
            .find(doc! {}, None)
            .await
            .map_err(UmtError::MongoFind)?;

        let mut configs = HashMap::new();

        while let Some(config) = cursor.try_next().await.map_err(UmtError::MongoReadCursor)? {
            configs.insert(config.id(), config.data);
        }

        Ok(configs)
    }

    pub async fn watch_configs(&self, tx: &Sender<Event>) -> Result<()> {
        let mut change_stream = self
            .database
            .collection::<Config>(CONFIGS)
            .watch(vec![], None)
            .await
            .map_err(UmtError::MongoWatchChangeStream)?;

        while let Some(Ok(_)) = change_stream.next().await {
            _ = tx.send(Event::Configs).await;
        }

        Ok(())
    }

    /* USERS */

    pub async fn create_user(&self, user: &User) -> Result<Option<ObjectId>> {
        self.database
            .collection(USERS)
            .insert_one(user.clone().into_create_db(), None)
            .await
            .map(|t| t.inserted_id.as_object_id())
            .map_err(UmtError::MongoInsertOne)
    }

    pub async fn read_user_by_username(&self, username: &str) -> Result<Option<User>> {
        self.database
            .collection::<User>(USERS)
            .find_one(doc! { "username": username }, None)
            .await
            .map_err(UmtError::MongoFindOne)
    }

    pub async fn update_user_by_id(&self, id: impl AsRef<str>, user: &User) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<User>(USERS)
            .update_one(
                doc! { "_id": _id },
                doc! { "$set": bson::to_bson(&user.clone().into_update_db()).unwrap() },
                None,
            )
            .await
            .map(|_| ())
            .map_err(UmtError::MongoUpdateOne)
    }

    pub async fn delete_user_by_id(&self, id: impl AsRef<str>) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<User>(USERS)
            .delete_one(doc! { "_id": _id }, None)
            .await
            .map(|_| ())
            .map_err(UmtError::MongoDeleteOne)
    }

    pub async fn read_all_users(&self) -> Result<(HashMap<ObjectId, User>, HashMap<String, User>)> {
        let mut cursor = self
            .database
            .collection::<User>(USERS)
            .find(
                doc! {},
                Some(FindOptions::builder().sort(doc! { "username": 1 }).build()),
            )
            .await
            .map_err(UmtError::MongoFind)?;

        let mut users = HashMap::new();
        let mut users_by_username = HashMap::new();

        while let Some(user) = cursor.try_next().await.map_err(UmtError::MongoReadCursor)? {
            users.insert(user.id(), user.clone());
            users_by_username.insert(user.username.clone(), user);
        }

        Ok((users, users_by_username))
    }

    pub async fn watch_users(&self, tx: &Sender<Event>) -> Result<()> {
        let mut change_stream = self
            .database
            .collection::<User>(USERS)
            .watch(vec![], None)
            .await
            .map_err(UmtError::MongoWatchChangeStream)?;

        while let Some(Ok(_)) = change_stream.next().await {
            _ = tx.send(Event::Users).await;
        }

        Ok(())
    }

    /* ROLES */

    pub async fn create_role(&self, role: &Role) -> Result<Option<ObjectId>> {
        self.database
            .collection(ROLES)
            .insert_one(RoleDB::from(role), None)
            .await
            .map(|t| t.inserted_id.as_object_id())
            .map_err(UmtError::MongoInsertOne)
    }

    pub async fn read_role_by_name(&self, name: &str) -> Result<Option<Role>> {
        self.database
            .collection(ROLES)
            .find_one(doc! { "name": name }, None)
            .await
            .map_err(UmtError::MongoFindOne)
    }

    pub async fn update_role_by_id(&self, id: impl AsRef<str>, role: &Role) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<Role>(ROLES)
            .update_one(
                doc! { "_id": _id },
                doc! { "$set": bson::to_bson(&RoleDB::from(role)).unwrap() },
                None,
            )
            .await
            .map(|_| ())
            .map_err(UmtError::MongoUpdateOne)
    }

    pub async fn delete_role_by_id(&self, id: impl AsRef<str>) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<Role>(ROLES)
            .delete_one(doc! { "_id": _id }, None)
            .await
            .map(|_| ())
            .map_err(UmtError::MongoDeleteOne)
    }

    pub async fn read_all_roles(&self) -> Result<(HashMap<ObjectId, Role>, HashMap<String, Role>)> {
        let mut cursor = self
            .database
            .collection::<Role>(ROLES)
            .find(doc! {}, None)
            .await
            .map_err(UmtError::MongoFind)?;

        let mut roles = HashMap::new();
        let mut roles_by_name = HashMap::new();

        while let Some(role) = cursor.try_next().await.map_err(UmtError::MongoReadCursor)? {
            roles.insert(role.id(), role.clone());
            roles_by_name.insert(role.name.clone(), role);
        }

        Ok((roles, roles_by_name))
    }

    pub async fn watch_roles(&self, tx: &Sender<Event>) -> Result<()> {
        let mut change_stream = self
            .database
            .collection::<Role>(ROLES)
            .watch(vec![], None)
            .await
            .map_err(UmtError::MongoWatchChangeStream)?;

        while let Some(Ok(_)) = change_stream.next().await {
            _ = tx.send(Event::Roles).await;
        }

        Ok(())
    }

    /* APPS */

    pub async fn create_app(&self, app: &App) -> Result<Option<ObjectId>> {
        self.database
            .collection(APPS)
            .insert_one(AppDB::from(app), None)
            .await
            .map(|t| t.inserted_id.as_object_id())
            .map_err(UmtError::MongoInsertOne)
    }

    pub async fn read_app_by_name(&self, name: &str) -> Result<Option<App>> {
        self.database
            .collection(APPS)
            .find_one(doc! { "name": name }, None)
            .await
            .map_err(UmtError::MongoFindOne)
    }

    pub async fn read_all_apps(&self) -> Result<HashMap<ObjectId, App>> {
        let mut cursor = self
            .database
            .collection::<App>(APPS)
            .find(doc! {}, None)
            .await
            .map_err(UmtError::MongoFind)?;

        let mut apps = HashMap::new();

        while let Some(app) = cursor.try_next().await.map_err(UmtError::MongoReadCursor)? {
            apps.insert(app.id(), app);
        }

        Ok(apps)
    }

    pub async fn update_app_by_id(&self, id: impl AsRef<str>, app: &App) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<App>(APPS)
            .update_one(
                doc! { "_id": _id },
                doc! { "$set": bson::to_bson(&AppDB::from(app)).unwrap() },
                None,
            )
            .await
            .map(|_| ())
            .map_err(UmtError::MongoUpdateOne)
    }

    pub async fn delete_app_by_id(&self, id: impl AsRef<str>) -> Result<()> {
        let _id = ObjectId::parse_str(id).map_err(UmtError::ParseObjectId)?;

        self.database
            .collection::<App>(APPS)
            .delete_one(doc! { "_id": _id }, None)
            .await
            .map(|_| ())
            .map_err(UmtError::MongoDeleteOne)
    }

    pub async fn watch_apps(&self, tx: &Sender<Event>) -> Result<()> {
        let mut change_stream = self
            .database
            .collection::<App>(APPS)
            .watch(vec![], None)
            .await
            .map_err(UmtError::MongoWatchChangeStream)?;

        while let Some(Ok(_)) = change_stream.next().await {
            _ = tx.send(Event::Apps).await;
        }

        Ok(())
    }

    /* TOKENS */

    pub async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<()> {
        self.database
            .collection::<RefreshToken>(TOKENS)
            .insert_one(refresh_token, None)
            .await
            .map(|_| ())
            .map_err(UmtError::MongoInsertOne)
    }

    pub async fn read_refresh_token(
        &self,
        refresh_token: &str,
        username: &str,
    ) -> Result<Option<RefreshToken>> {
        self.database
            .collection(TOKENS)
            .find_one(doc! { "token": refresh_token, "username": username }, None)
            .await
            .map_err(UmtError::MongoFindOne)
    }

    pub async fn delete_refresh_token(
        &self,
        refresh_token: &str,
        username: &str,
    ) -> Result<Option<RefreshToken>> {
        self.database
            .collection(TOKENS)
            .find_one_and_delete(doc! { "token": refresh_token, "username": username }, None)
            .await
            .map_err(UmtError::MongoDeleteOne)
    }

    /* INIT */

    pub async fn init(&self) -> Result<()> {
        // Create web config.
        if self.read_config(TOKEN_CONFIG).await?.is_none() {
            self.create_config(&Config::new_token()).await?;
        }

        // Create apps indexes.
        self.database
            .collection::<App>(APPS)
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "name": 1, "version": 1 })
                    .options(Some(IndexOptions::builder().unique(true).build()))
                    .build(),
                None,
            )
            .await
            .map_err(UmtError::MongoCreateIndex)?;

        // Create users indexes.
        self.database
            .collection::<User>(USERS)
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "username": 1 })
                    .options(Some(IndexOptions::builder().unique(true).build()))
                    .build(),
                None,
            )
            .await
            .map_err(UmtError::MongoCreateIndex)?;

        // Create roles indexes.
        self.database
            .collection::<Role>(ROLES)
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "name": 1 })
                    .options(Some(IndexOptions::builder().unique(true).build()))
                    .build(),
                None,
            )
            .await
            .map_err(UmtError::MongoCreateIndex)?;

        // Create tokens indexes.
        self.database
            .collection::<SessionToken>(TOKENS)
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "username": 1, "token": 1 })
                    .build(),
                None,
            )
            .await
            .map_err(UmtError::MongoCreateIndex)?;

        let expire_after = std::time::Duration::from_secs(43200);

        self.database
            .collection::<SessionToken>(TOKENS)
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "createdAt": 1 })
                    .options(Some(
                        IndexOptions::builder()
                            .expire_after(Some(expire_after))
                            .build(),
                    ))
                    .build(),
                None,
            )
            .await
            .map_err(UmtError::MongoCreateIndex)?;

        // Create local app.
        if self.read_app_by_name(LOCAL_APP).await?.is_none() {
            let app = App::default();
            let app_id = self.create_app(&app).await?;

            let mut role_id = None;

            // Create local role.
            if let Some(t) = app_id {
                if self.read_role_by_name(LOCAL_ROLE).await?.is_none() {
                    let role = Role {
                        app: t,
                        items: app.default_role,
                        ..Default::default()
                    };

                    role_id = self.create_role(&role).await?;
                }
            }

            // Create admin user.
            if let Some(t) = role_id {
                if self.read_user_by_username(ADMIN_USERNAME).await?.is_none() {
                    let user = User {
                        roles: vec![t],
                        ..Default::default()
                    };

                    self.create_user(&user.hash_password()).await?;
                }
            }
        }

        Ok(())
    }
}
