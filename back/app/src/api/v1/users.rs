use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use super::{Output, Example, Status};
use crate::dao::Memory;
use crate::tokens::SessionToken;
use crate::users::{User, UsersVec};
use crate::{Shared, UsermanError};

impl Example for User {
    fn example() -> Self {
        Self::default()
    }
}

impl Example for UsersVec {
    fn example() -> Self {
        Self(vec![User::default()])
    }
}

#[utoipa::path(
    post, 
    path = "/api/v1/users",
    request_body(content = User, example = json!(User::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Create user successfully", 
            body = StatusGeneric,
            example = json!(Status::<User>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Create user with error",
            body = StatusGeneric,
            example = json!(Status::<User>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let create = value!(items, "/users/create.boolean");

    validate_bool!(create);

    match shared.dao.create_user(&payload.none_password()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/users/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read user successfully", 
            body = StatusUser,
            example = json!(Status::<User>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read user with error",
            body = StatusUser,
            example = json!(Status::<User>::example_bad_request())
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
    let read = value!(items, "/users/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UsermanError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.users.get_by_id(&object_id).await {
        Some(t) => Output::Success(t.hide_password()),
        None => Output::Done,
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/users",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read users successfully", 
            body = StatusUsers,
            example = json!(Status::<UsersVec>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read users with error",
            body = StatusUsers,
            example = json!(Status::<UsersVec>::example_bad_request())
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
    let read = value!(items, "/users/read.boolean");

    validate_bool!(read);

    let values: Vec<User> = shared.users.get_all().await.iter_mut().map(|user| {
        user.to_owned().hide_password()
    }).collect();

    Output::Success(values)
}

#[utoipa::path(
    put, 
    path = "/api/v1/users/<id>",
    request_body(content = User, example = json!(User::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Update user successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Update user with error",
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
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let update = value!(items, "/users/update.boolean");

    validate_bool!(update);

    match shared
        .dao
        .update_user_by_id(id.as_str(), &payload.none_password())
        .await
    {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/users/<id>/reset",
    responses(
        (
            status = StatusCode::OK, 
            description = "Reset user password successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Reset user password with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn reset(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let update = value!(items, "/users/update.boolean");

    validate_bool!(update);

    match shared.dao.reset_user_password_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    delete, 
    path = "/api/v1/users/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Delete user successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Delete user with error",
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
    let delete = value!(items, "/users/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_user_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/usernames/<username>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read user successfully", 
            body = StatusUser,
            example = json!(Status::<User>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read user with error",
            body = StatusUser,
            example = json!(Status::<User>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn username(
    username: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let read = value!(items, "/users/read.boolean");

    validate_bool!(read);

    match shared.users.get(&username).await {
        Some(t) => Output::Success(t.hide_password()),
        None => Output::Done,
    }
}
