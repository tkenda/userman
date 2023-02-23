use axum::{extract::Json, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};

use crate::dao::Memory;
use crate::tokens::{Claims, RefreshToken};
use crate::{Shared, UmtError};

use super::Output;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginReq {
    username: String,
    password: String,
    device: Option<String>,
    location: Option<Vec<f64>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginRes {
    access_token: String,
    refresh_token: String,
}

pub(crate) async fn login(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<LoginReq>,
) -> impl IntoResponse {
    match shared.users.get(&payload.username).await {
        Some(t) if t.verify(payload.password) => {
            let app = "APP".to_string();
            let username = t.username;

            /* access token */

            let duration = shared.keys.duration().await;
            let mut roles_names = vec![];

            for role_id in t.roles {
                if let Some(role) = shared.roles.get_by_id(&role_id).await {
                    roles_names.push(role.name);
                }
            }

            let claims = Claims::new(&username, roles_names, duration);

            let encoding_key = shared.keys.encoding_key().await;

            let access_token = match claims.encode(&encoding_key) {
                Ok(t) => t,
                Err(err) => return Output::Failure(err),
            };

            /* refresh token */

            let refresh_token =
                RefreshToken::build(username, app, payload.device, payload.location);

            if let Err(err) = shared.dao.create_refresh_token(&refresh_token).await {
                return Output::Failure(err);
            }

            Output::Success(LoginRes {
                access_token,
                refresh_token: refresh_token.to_string(),
            })
        }
        _ => Output::Failure(UmtError::InvalidCredentials),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RefreshReq {
    username: String,
    refresh_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RefreshRes {
    access_token: String,
}

pub(crate) async fn refresh(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<RefreshReq>,
) -> impl IntoResponse {
    match shared
        .dao
        .read_refresh_token(&payload.refresh_token, &payload.username)
        .await
    {
        Ok(None) => Output::Failure(UmtError::InvalidToken),
        Ok(_) => match shared.users.get(&payload.username).await {
            Some(t) => {
                let duration = shared.keys.duration().await;
                let mut roles_names = vec![];

                for role_id in t.roles {
                    if let Some(role) = shared.roles.get_by_id(&role_id).await {
                        roles_names.push(role.name);
                    }
                }

                let claims = Claims::new(&t.username, roles_names, duration);

                let encoding_key = shared.keys.encoding_key().await;

                let access_token = match claims.encode(&encoding_key) {
                    Ok(t) => t,
                    Err(err) => return Output::Failure(err),
                };

                Output::Success(RefreshRes { access_token })
            }
            _ => Output::Failure(UmtError::InvalidUsername),
        },
        Err(err) => Output::Failure(err),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LogoutReq {
    username: String,
    refresh_token: String,
}

pub(crate) async fn logout(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<LogoutReq>,
) -> impl IntoResponse {
    match shared
        .dao
        .delete_refresh_token(&payload.refresh_token, &payload.username)
        .await
    {
        Ok(Some(_)) => Output::<()>::Done,
        Ok(None) => Output::Unauthorized(UmtError::InvalidCredentials),
        Err(err) => Output::Failure(err),
    }
}
