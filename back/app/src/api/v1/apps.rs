use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use userman_auth::apps::{App, AppsVec};
use userman_auth::roles::RoleItems;

use super::{Output, Example, Status};
use crate::dao::Memory;
use crate::error::UsermanError;
use crate::tokens::SessionToken;
use crate::Shared;

impl Example for App {
    fn example() -> Self {
        Self {
            id: None,
            name: "NAME".to_string(),
            version: 1,
            default_role: RoleItems::local(),
            created_at: None,
            updated_at: None,
        }
    }
}

impl Example for AppsVec {
    fn example() -> Self {
        Self(vec!(
            App {
                id: None,
                name: "NAME".to_string(),
                version: 1,
                default_role: RoleItems::local(),
                created_at: None,
                updated_at: None,
            }
        ))
    }
}

#[utoipa::path(
    post, 
    path = "/api/v1/apps",
    request_body(content = App, example = json!(App::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Create app successfully", 
            body = StatusGeneric,
            example = json!(Status::<App>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Create app with error",
            body = StatusGeneric,
            example = json!(Status::<App>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<App>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let create = value!(items, "/apps/create.boolean");

    validate_bool!(create);

    match shared.dao.create_app(&payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/apps/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read app successfully", 
            body = StatusApp,
            example = json!(Status::<App>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read app with error",
            body = StatusApp,
            example = json!(Status::<App>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn read(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let read = value!(items, "/apps/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UsermanError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.apps.get_by_id(&object_id).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/apps",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read apps successfully", 
            body = StatusApp,
            example = json!(Status::<AppsVec>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read apps with error",
            body = StatusApp,
            example = json!(Status::<AppsVec>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn read_all(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let read = value!(items, "/apps/read.boolean");

    validate_bool!(read);

    let values = shared.apps.get_all().await;
    Output::Success(values)
}

#[utoipa::path(
    put, 
    path = "/api/v1/apps/<id>",
    request_body(content = App, example = json!(App::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Update app successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Update app with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn update(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<App>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let update = value!(items, "/apps/update.boolean");

    validate_bool!(update);

    match shared.dao.update_app_by_id(id.as_str(), &payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    delete, 
    path = "/api/v1/apps/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Delete app successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Delete app with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn delete(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let delete = value!(items, "/apps/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_app_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}
