use axum::{extract::Json, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use userman_auth::roles::RoleItems;
use utoipa::ToSchema;

use super::{Output, Example, Status};
use crate::dao::Memory;
use crate::tokens::{Claims, RefreshToken};
use crate::{Shared, UsermanError};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginReq {
    username: String,
    password: String,
    device: Option<String>,
    location: Option<Vec<f64>>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginRes {
    access_token: String,
    refresh_token: String,
    permissions: RoleItems,
}

impl Example for LoginRes {
    fn example() -> Self {
        Self {
            access_token: "ACCESS_TOKEN".to_string(),
            refresh_token: "REFRESH_TOKEN".to_string(),
            permissions: RoleItems::local(),
        }
    }
}

#[utoipa::path(
    post, 
    path = "/api/v1/login",
    request_body = LoginReq,
    responses(
        (
            status = StatusCode::OK, 
            description = "Login successfully", 
            body = StatusLoginRes,
            example = json!(Status::<LoginRes>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Login with error",
            body = StatusLoginRes,
            example = json!(Status::<LoginRes>::example_bad_request())
        )
    )
)]
pub(crate) async fn login(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<LoginReq>,
) -> impl IntoResponse {
    match shared.users.get(&payload.username).await {
        Some(t) => {
            match t.verify(payload.password) {
                Some(true) if t.enabled => {
                    /* access token */

                    let duration = shared.keys.duration().await;
                    let mut roles_names = vec![];

                    for role_id in &t.roles {
                        if let Some(role) = shared.roles.get_by_id(role_id).await {
                            roles_names.push(role.name);
                        }
                    }

                    let claims = Claims::new(&t.username, roles_names.clone(), duration);

                    let encoding_key = shared.keys.encoding_key().await;

                    let access_token = match claims.encode(&encoding_key) {
                        Ok(t) => t,
                        Err(err) => return Output::Failure(err),
                    };

                    /* refresh token */

                    let refresh_token =
                        RefreshToken::build(t.id(), "userman", payload.device, payload.location);

                    if let Err(err) = shared.dao.create_refresh_token(&refresh_token).await {
                        return Output::Failure(err);
                    }

                    /* permissions */

                    let permissions = shared.auth.permissions(roles_names).await;

                    Output::Success(LoginRes {
                        access_token,
                        refresh_token: refresh_token.to_string(),
                        permissions,
                    })
                },
                Some(true) => Output::Failure(UsermanError::DisabledUser),
                Some(false) => Output::Failure(UsermanError::InvalidCredentials),
                None => Output::Failure(UsermanError::UninitializedPassword),
            }
        }
        _ => Output::Failure(UsermanError::InvalidCredentials),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RefreshReq {
    username: String,
    refresh_token: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RefreshRes {
    access_token: String,
}

impl Example for RefreshRes {
    fn example() -> Self {
        Self {
            access_token: "NEW_ACCESS_TOKEN".to_string(),
        }
    }
}

#[utoipa::path(
    post, 
    path = "/api/v1/refresh",
    request_body = RefreshReq,
    responses(
        (
            status = StatusCode::OK, 
            description = "Refresh successfully", 
            body = StatusLoginRes,
            example = json!(Status::<RefreshRes>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Refresh with error",
            body = StatusLoginRes,
            example = json!(Status::<RefreshRes>::example_bad_request())
        )
    )
)]
pub(crate) async fn refresh(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<RefreshReq>,
) -> impl IntoResponse {
    match shared
        .dao
        .read_refresh_token(&payload.refresh_token, &payload.username)
        .await
    {
        Ok(None) => Output::Failure(UsermanError::InvalidToken),
        Ok(_) => match shared.users.get(&payload.username).await {
            Some(t) if t.enabled => {
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
            Some(_) => Output::Failure(UsermanError::DisabledUser),
            _ => Output::Failure(UsermanError::InvalidUsername),
        },
        Err(err) => Output::Failure(err),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LogoutReq {
    username: String,
    refresh_token: String,
}

#[utoipa::path(
    post, 
    path = "/api/v1/logout",
    request_body = LogoutReq,
    responses(
        (
            status = StatusCode::OK, 
            description = "Logout successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Logout with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    )
)]
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
        Ok(None) => Output::Unauthorized(UsermanError::InvalidCredentials),
        Err(err) => Output::Failure(err),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResetReq {
    id: String,
    password: String,
}

#[utoipa::path(
    post, 
    path = "/api/v1/reset",
    request_body = ResetReq,
    responses(
        (
            status = StatusCode::OK, 
            description = "Reset password successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Reset password with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    )
)]
pub(crate) async fn reset(
    Extension(shared): Extension<Shared>,
    Json(payload): Json<ResetReq>,
) -> impl IntoResponse {
    match shared
        .dao
        .update_user_password_by_id(&payload.id, &payload.password)
        .await
    {
        Ok(()) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}