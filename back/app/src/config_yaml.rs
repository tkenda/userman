//! Main app configuration readed from config.yaml.
use haikunator::Haikunator;
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::IpAddr;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::dao::Dao;
use crate::logger::LogsLevel;
use crate::{Result, UsermanError};

fn default_mongo_db_uri() -> String {
    String::from("mongodb://localhost:27017")
}

fn default_mongo_db_db_name() -> String {
    String::from("userman")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoDB {
    #[serde(default = "default_mongo_db_uri")]
    pub uri: String,

    #[serde(default = "default_mongo_db_db_name")]
    pub db_name: String,
}

impl Default for MongoDB {
    fn default() -> Self {
        Self {
            uri: default_mongo_db_uri(),
            db_name: default_mongo_db_db_name(),
        }
    }
}

impl From<MongoDB> for userman_auth::MongoDB {
    fn from(src: MongoDB) -> Self {
        userman_auth::MongoDB {
            uri: src.uri.to_owned(),
            db_name: src.db_name,
            client_name: "local".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tls {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub certs: String,

    #[serde(default)]
    pub key: String,
}

fn default_front_public_url() -> String {
    String::new()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Front {
    #[serde(default = "default_front_public_url")]
    pub public_url: String,
}

impl Default for Front {
    fn default() -> Self {
        Self {
            public_url: default_front_public_url(),
        }
    }
}

fn default_name() -> String {
    Haikunator::default().haikunate()
}

fn default_ip() -> IpAddr {
    "0.0.0.0".parse().unwrap()
}

fn default_port() -> u16 {
    8090
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigYAML {
    #[serde(default = "default_name")]
    pub name: String,

    #[serde(default = "default_ip")]
    pub ip: IpAddr,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub logs: LogsLevel,

    #[serde(default)]
    pub mongo_db: MongoDB,

    #[serde(default)]
    pub tls: Tls,

    #[serde(default)]
    pub front: Front,
}

impl Default for ConfigYAML {
    fn default() -> Self {
        Self {
            name: default_name(),
            ip: default_ip(),
            port: default_port(),
            mongo_db: MongoDB::default(),
            tls: Tls::default(),
            logs: LogsLevel::default(),
            front: Front::default(),
        }
    }
}

impl ConfigYAML {
    pub async fn read_or_create_file<T: AsRef<Path> + Display>(path: T) -> Result<Self> {
        match File::open(&path).await {
            Ok(mut reader) => {
                let mut content = String::new();
                reader
                    .read_to_string(&mut content)
                    .await
                    .map_err(|err| UsermanError::StdIoError(err.to_string()))?;
                serde_yaml::from_str(&content).map_err(|err| UsermanError::YAMLFile(err.to_string()))
            }
            Err(err) => {
                debug!("{}", err);
                info!("Missing {} file. Creating new..", path);

                let mut reader = File::create(path)
                    .await
                    .map_err(|err| UsermanError::StdIoError(err.to_string()))?;
                let content: String = serde_yaml::to_string(&Self::default()).unwrap();
                reader
                    .write_all(content.as_bytes())
                    .await
                    .map_err(|err| UsermanError::StdIoError(err.to_string()))?;

                Ok(Self::default())
            }
        }
    }

    pub async fn dao(&self) -> Result<Dao> {
        let mut client_options = ClientOptions::parse(&self.mongo_db.uri)
            .await
            .map_err(UsermanError::MongoParseUri)?;

        client_options.app_name = Some(self.name.to_owned());

        let client = Client::with_options(client_options).map_err(UsermanError::MongoCreateClient)?;

        let database = client.database(&self.mongo_db.db_name);

        Ok(Dao::new(database))
    }
}
