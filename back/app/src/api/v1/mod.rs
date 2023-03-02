pub mod apps;
pub mod roles;
pub mod sessions;
pub mod users;

use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::Router;
use reqwest::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use userman_auth::apps::{App, AppsVec};
use userman_auth::roles::{Role, RolesVec};

use crate::users::{User, UsersVec};
use crate::{Result, UsermanError};

use sessions::{LoginRes, RefreshRes};

static DONE: &str = "done";
static ERROR: &str = "error";

#[derive(Serialize, ToSchema)]
pub struct StringsVec(pub Vec<String>);

#[derive(ToSchema)]
#[aliases(
    StatusGeneric = Status<String>,
    StatusLoginRes = Status<LoginRes>,
    StatusRefreshRes = Status<RefreshRes>,
    StatusStrings = Status<StringsVec>,
    StatusApp = Status<App>,
    StatusApps = Status<AppsVec>,
    StatusRole = Status<Role>,
    StatusRoles = Status<RolesVec>,
    StatusUser = Status<User>,
    StatusUsers = Status<UsersVec>,
)]
#[derive(Serialize)]
pub(crate) struct Status<T>
where
    T: Serialize,
{
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u8>,
}

impl<T: Serialize> Status<T> {
    fn into_string(self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(UsermanError::CreateJSON)
    }
}

impl<T: Serialize> Status<T> {
    fn example_empty_ok() -> Self {
        Self {
            status: DONE,
            data: None,
            error: None,
            code: None,
        }
    }

    fn example_bad_request() -> Self {
        Self {
            status: ERROR,
            data: None,
            error: Some("ERROR_MESSAGE".to_string()),
            code: None,
        }
    }
}

pub trait Example {
    fn example() -> Self;
}

impl<T: Serialize + Example> Status<T> {
    fn example_ok() -> Self {
        Self {
            status: DONE,
            data: Some(T::example()),
            error: None,
            code: None,
        }
    }
}

pub enum Output<T: Serialize> {
    Done,
    Success(T),
    Failure(UsermanError),
    Unauthorized(UsermanError),
}

impl<T: Serialize> IntoResponse for Output<T> {
    fn into_response(self) -> Response {
        match self {
            Output::Done => {
                let resp: Status<()> = Status {
                    status: DONE,
                    data: None,
                    error: None,
                    code: None,
                };

                (StatusCode::OK, resp.into_string().unwrap()).into_response()
            }
            Output::Success(ref s) => {
                let resp = Status {
                    status: DONE,
                    data: Some(s),
                    error: None,
                    code: None,
                };

                match resp.into_string() {
                    Ok(t) => (StatusCode::OK, t).into_response(),
                    Err(err) => {
                        let resp_err: Status<()> = Status {
                            status: ERROR,
                            data: None,
                            error: Some(err.to_string()),
                            code: None,
                        };

                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            resp_err.into_string().unwrap(),
                        )
                            .into_response()
                    }
                }
            }
            Output::Failure(ref f) => {
                let resp_err: Status<()> = match f.code_number() {
                    Some(t) => Status {
                        status: ERROR,
                        data: None,
                        error: Some(f.to_string()),
                        code: Some(t),
                    },
                    None => Status {
                        status: ERROR,
                        data: None,
                        error: Some(f.to_string()),
                        code: None,
                    },
                };

                (StatusCode::BAD_REQUEST, resp_err.into_string().unwrap()).into_response()
            }
            Output::Unauthorized(f) => {
                let resp_err: Status<()> = match f.code_number() {
                    Some(t) => Status {
                        status: ERROR,
                        data: None,
                        error: Some(f.to_string()),
                        code: Some(t),
                    },
                    None => Status {
                        status: ERROR,
                        data: None,
                        error: Some(f.to_string()),
                        code: None,
                    },
                };

                (StatusCode::UNAUTHORIZED, resp_err.into_string().unwrap()).into_response()
            }
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(sessions::login))
        .route("/refresh", post(sessions::refresh))
        .route("/logout", post(sessions::logout))
        .route("/reset", post(sessions::reset))
        // apps
        .route("/apps", post(apps::create).get(apps::read_all))
        .route(
            "/apps/:id",
            put(apps::update).delete(apps::delete).get(apps::read),
        )
        // roles
        .route("/roles", post(roles::create).get(roles::read_all))
        .route("/roles/:id/sync", get(roles::sync))
        .route("/rolenames/:name", get(roles::name))
        .route("/rolenames", get(roles::read_all_names))
        .route(
            "/roles/:id",
            put(roles::update).delete(roles::delete).get(roles::read),
        )
        // users
        .route("/users", post(users::create).get(users::read_all))
        .route("/usernames/:username", get(users::username))
        .route(
            "/users/:id",
            put(users::update).delete(users::delete).get(users::read),
        )
        .route("/users/:id/reset", get(users::reset))
}
