use axum::{response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

use auth::AuthError;

#[derive(Debug, Error)]
pub enum UmtError {
    #[error("Can't create SIGINT stream. {0}")]
    SignalInterrupt(std::io::Error),
    #[error("Can't create SIGTERM stream. {0}")]
    SignalTerminate(std::io::Error),
    #[error("Error blocking main thread with SIGINT.")]
    InterruptBlocking,
    #[error("Error blocking main thread with SIGTERM.")]
    TerminateBlocking,
    #[error("{0}")]
    StdIoError(String),
    #[error("Error parsing YAML file. {0}")]
    YAMLFile(String),
    #[error("Error reading PEM file. {0}")]
    PEMFile(String),
    #[error("Web server error. {0}")]
    WebServer(String),
    #[error("Could not parse MongoDB URI. {0}")]
    MongoParseUri(mongodb::error::Error),
    #[error("Could not create MongoDB client. {0}")]
    MongoCreateClient(mongodb::error::Error),
    #[error("Could not create MongoDB collection. {0}")]
    MongoCreateCollection(mongodb::error::Error),
    #[error("Could not read MongoDB cursor. {0}")]
    MongoReadCursor(mongodb::error::Error),
    #[error("Could not watch a MongoDB watch stream. {0}")]
    MongoWatchChangeStream(mongodb::error::Error),
    #[error("MongoDB find API error. {0}")]
    MongoFind(mongodb::error::Error),
    #[error("MongoDB find one API error. {0}")]
    MongoFindOne(mongodb::error::Error),
    #[error("MongoDB insert one API error. {0}")]
    MongoInsertOne(mongodb::error::Error),
    #[error("MongoDB update one API error. {0}")]
    MongoUpdateOne(mongodb::error::Error),
    #[error("MongoDB delete one API error. {0}")]
    MongoDeleteOne(mongodb::error::Error),
    #[error("MongoDB create index API error. {0}")]
    MongoCreateIndex(mongodb::error::Error),
    #[error("Could not get {0} configuration.")]
    GetConfig(&'static str),
    #[error("JWT encode: {0}")]
    JWTEncode(jsonwebtoken::errors::Error),
    #[error("JWT decode: {0}")]
    JWTDecode(jsonwebtoken::errors::Error),
    #[error("Could not create JSON. {0}")]
    CreateJSON(serde_json::Error),
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("Invalid username.")]
    InvalidUsername,
    #[error("Invalid token.")]
    InvalidToken,
    #[error("Parse MongoDB ObjectId UUID. {0}")]
    ParseObjectId(mongodb::bson::oid::Error),
    #[error("Role not found.")]
    RoleNotFound,
    #[error("App not found.")]
    AppNotFound,
    #[error("Auth error. {0}")]
    Auth(#[from] AuthError),
    #[error("Unauthorized")]
    Unauthorized,
}

impl UmtError {
    pub fn code_number(&self) -> Option<u8> {
        match self {
            Self::InvalidToken => Some(1),
            _ => None,
        }
    }
}

impl IntoResponse for UmtError {
    fn into_response(self) -> Response {
        let body = match self.code_number() {
            Some(t) => {
                json!({
                    "status": "error",
                    "error": self.to_string(),
                    "code": t,
                })
            }
            None => {
                json!({
                    "status": "error",
                    "error": self.to_string(),
                })
            }
        };

        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}