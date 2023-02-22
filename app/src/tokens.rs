use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use rand::distributions::{Alphanumeric, DistString};
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicI64, Arc};

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::configs::{ConfigData, TOKEN_CONFIG};
use crate::dao::{Dao, Memory};
use crate::{Result, UmtError};

#[derive(Clone)]
pub struct Keys {
    encoding_key: Arc<RwLock<EncodingKey>>,
    decoding_key: Arc<RwLock<DecodingKey>>,
    duration: Arc<AtomicI64>,
}

impl Keys {
    pub async fn encoding_key(&self) -> EncodingKey {
        self.encoding_key.read().await.clone()
    }

    pub async fn decoding_key(&self) -> DecodingKey {
        self.decoding_key.read().await.clone()
    }

    pub async fn duration(&self) -> i64 {
        self.duration.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl Memory<ConfigData, ObjectId> for Keys {
    async fn load(dao: &Dao) -> Result<Self> {
        let web_config = match dao.read_config(TOKEN_CONFIG).await? {
            Some(t) => t.unwrap_token()?,
            _ => return Err(UmtError::GetConfig(TOKEN_CONFIG)),
        };

        let encoding_key = EncodingKey::from_secret(web_config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(web_config.secret.as_bytes());
        let duration = web_config.duration;

        Ok(Self {
            encoding_key: Arc::new(RwLock::new(encoding_key)),
            decoding_key: Arc::new(RwLock::new(decoding_key)),
            duration: Arc::new(AtomicI64::from(duration)),
        })
    }

    async fn reload(&self, dao: &Dao) -> Result<()> {
        let web_config = match dao.read_config(TOKEN_CONFIG).await? {
            Some(t) => t.unwrap_token()?,
            _ => return Err(UmtError::GetConfig(TOKEN_CONFIG)),
        };

        let encoding_key = EncodingKey::from_secret(web_config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(web_config.secret.as_bytes());
        let duration = web_config.duration;

        {
            let mut value = self.encoding_key.write().await;
            *value = encoding_key;
        }

        {
            let mut value = self.decoding_key.write().await;
            *value = decoding_key;
        }

        self.duration.store(duration, Ordering::SeqCst);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    roles: Vec<String>,
}

impl Claims {
    pub fn new(subject: &str, roles: Vec<String>, duration: i64) -> Self {
        let expiration = Utc::now() + Duration::seconds(duration);

        Self {
            sub: subject.to_string(),
            exp: expiration.timestamp() as usize,
            roles,
        }
    }

    pub fn encode(&self, encoding_key: &EncodingKey) -> Result<String> {
        encode(&Header::default(), &self, encoding_key).map_err(UmtError::JWTEncode)
    }

    pub fn decode(token: &str, decoding_key: &DecodingKey) -> Result<Self> {
        let token = decode::<Claims>(token, decoding_key, &Validation::default())
            .map_err(UmtError::JWTDecode)?;

        Ok(token.claims)
    }
}

#[derive(Debug)]
pub struct SessionToken(String);

#[async_trait]
impl<S> FromRequestParts<S> for SessionToken
where
    S: Send + Sync,
{
    type Rejection = UmtError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| UmtError::InvalidToken)?;

        Ok(Self(bearer.token().to_owned()))
    }
}

impl SessionToken {
    pub async fn role_names(&self, keys: &Keys) -> Result<Vec<String>> {
        let decoding_key = keys.decoding_key().await;
        let claims = Claims::decode(&self.0, &decoding_key)?;
        Ok(claims.roles)
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    username: String,
    token: String,
    app: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Vec<f64>>,
    created_at: DateTime,
}

impl ToString for RefreshToken {
    fn to_string(&self) -> String {
        self.token.to_string()
    }
}

impl RefreshToken {
    pub fn build(
        username: String,
        app: String,
        device: Option<String>,
        location: Option<Vec<f64>>,
    ) -> Self {
        Self {
            username,
            token: Alphanumeric.sample_string(&mut rand::thread_rng(), 128),
            app,
            device,
            location,
            created_at: DateTime::now(),
        }
    }
}
