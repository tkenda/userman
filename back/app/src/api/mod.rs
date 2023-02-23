use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use reqwest::StatusCode;
use serde::Serialize;
use std::net::SocketAddr;

use crate::{Result, Shared, UmtError};

macro_rules! roles {
    ($shared:ident, $token:ident) => {
        match $shared.role_items($token).await {
            Ok(t) => t,
            Err(err) => return Output::Unauthorized(err),
        }
    };
}

macro_rules! value {
    ($roles:ident, $path:expr) => {
        match $roles.find_value($path) {
            Ok(t) => t,
            Err(err) => return Output::Unauthorized(err.into()),
        }
    };
}

macro_rules! validate_bool {
    ($value:ident) => {
        use userman_auth::role::DataValue;

        match $value {
            DataValue::Boolean(true) => {}
            _ => return Output::Unauthorized(UmtError::Unauthorized),
        }
    };
}

mod apps;
mod roles;
mod sessions;
mod users;

#[derive(Serialize)]
struct Status<T>
where
    T: Serialize,
{
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> Status<T> {
    fn into_string(self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(UmtError::CreateJSON)
    }
}

#[derive(Serialize)]
struct InternalError {
    status: &'static str,
    error: String,
}

impl InternalError {
    fn into_string(self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(UmtError::CreateJSON)
    }
}

pub enum Output<T: Serialize> {
    Done,
    Success(T),
    Failure(UmtError),
    Unauthorized(UmtError),
}

impl<T: Serialize> IntoResponse for Output<T> {
    fn into_response(self) -> Response {
        match self {
            Output::Done => {
                let resp: Status<()> = Status {
                    status: "done",
                    data: None,
                };

                (StatusCode::OK, resp.into_string().unwrap()).into_response()
            }
            Output::Success(ref s) => {
                let resp = Status {
                    status: "done",
                    data: Some(s),
                };

                match resp.into_string() {
                    Ok(t) => (StatusCode::OK, t).into_response(),
                    Err(err) => {
                        let resp: InternalError = InternalError {
                            status: "error",
                            error: err.to_string(),
                        };

                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            resp.into_string().unwrap(),
                        )
                            .into_response()
                    }
                }
            }
            Output::Failure(f) => f.into_response(),
            Output::Unauthorized(f) => {
                let mut response = f.into_response();
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                response
            }
        }
    }
}

static A: &str = "/apps/:id";
static R: &str = "/roles/:id";
static U: &str = "/users/:id";

fn v1_routes() -> Router {
    Router::new()
        .route("/login", post(sessions::login))
        .route("/logout", post(sessions::logout))
        .route("/refresh", post(sessions::refresh))
        // apps
        .route("/apps", post(apps::create).get(apps::read_all))
        .route(A, put(apps::update).delete(apps::delete).get(apps::read))
        // roles
        .route("/roles", post(roles::create).get(roles::read_all))
        .route("/roles/:id/sync", get(roles::sync))
        .route("/rolename/:name", get(roles::name))
        .route("/rolenames", get(roles::read_all_names))
        .route(R, put(roles::update).delete(roles::delete).get(roles::read))
        // users
        .route("/users", post(users::create).get(users::read_all))
        .route("/username/:username", get(users::username))
        .route(U, put(users::update).delete(users::delete).get(users::read))
}

pub async fn run(shared: Shared) -> Result<()> {
    let config_yaml = shared.config_yaml.clone();
    let address = SocketAddr::new(config_yaml.ip, config_yaml.port);

    let app = Router::new()
        .nest(
            &format!("{}/api/v1", config_yaml.front.public_url),
            v1_routes(),
        )
        .layer(Extension(shared));

    match config_yaml.tls.enabled {
        true => {
            let rustls = RustlsConfig::from_pem_file(&config_yaml.tls.certs, &config_yaml.tls.key)
                .await
                .map_err(|err| UmtError::PEMFile(err.to_string()))?;

            axum_server::bind_rustls(address, rustls)
                .serve(app.into_make_service())
                .await
                .map_err(|err| UmtError::WebServer(err.to_string()))
        }
        false => axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .map_err(|err| UmtError::WebServer(err.to_string())),
    }
}
