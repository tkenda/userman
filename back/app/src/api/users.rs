use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use crate::dao::Memory;
use crate::tokens::SessionToken;
use crate::users::User;
use crate::{UmtError, Shared};

use super::Output;

pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let create = value!(users, "/users/create.boolean");

    validate_bool!(create);

    match shared.dao.create_user(&payload.hash_password()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn read(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let read = value!(users, "/users/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UmtError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.users.get_by_id(&object_id).await {
        Some(t) => Output::Success(t.hide_password()),
        None => Output::Done,
    }
}

pub(crate) async fn read_all(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let read = value!(users, "/users/read.boolean");

    validate_bool!(read);

    let mut values = vec![];

    for user in shared.users.get_all().await {
        values.push(user.hide_password());
    }

    Output::Success(values)
}

pub(crate) async fn update(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let update = value!(users, "/users/update.boolean");

    validate_bool!(update);

    match shared.dao.update_user_by_id(id.as_str(), &payload.hash_password()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn delete(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let delete = value!(users, "/users/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_user_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn username(
    username: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let users = roles!(shared, token);
    let read = value!(users, "/users/read.boolean");

    validate_bool!(read);

    match shared.users.get(&username).await {
        Some(t) => Output::Success(t.hide_password()),
        None => Output::Done,
    }
}