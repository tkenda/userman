use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use userman_auth::role::Role;

use crate::dao::Memory;
use crate::error::UmtError;
use crate::roles::RoleName;
use crate::tokens::SessionToken;
use crate::Shared;

use super::Output;

pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<Role>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let create = value!(roles, "/roles/create.boolean");

    validate_bool!(create);

    match shared.dao.create_role(&payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn read(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let read = value!(roles, "/roles/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UmtError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.roles.get_by_id(&object_id).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

pub(crate) async fn read_all(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let read = value!(roles, "/roles/read.boolean");

    validate_bool!(read);
    
    let values = shared.roles.get_all().await;
    Output::Success(values)
}

pub(crate) async fn read_all_names(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let read = value!(roles, "/roles/read.boolean");

    validate_bool!(read);

    let values: Vec<RoleName> = shared
        .roles
        .get_all()
        .await
        .into_iter()
        .map(|ref t| RoleName::from(t))
        .collect();

    Output::Success(values)
}

pub(crate) async fn update(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<Role>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let update = value!(roles, "/roles/update.boolean");

    validate_bool!(update);

    match shared.dao.update_role_by_id(id.as_str(), &payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn delete(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let delete = value!(roles, "/roles/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_role_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn name(
    name: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let read = value!(roles, "/roles/read.boolean");

    validate_bool!(read);

    match shared.roles.get(&name).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

pub(crate) async fn sync(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let roles = roles!(shared, token);
    let update = value!(roles, "/roles/update.boolean");

    validate_bool!(update);
    
    let object_id = match ObjectId::from_str(id.as_str()).map_err(UmtError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    let mut a_role = match shared.roles.get_by_id(&object_id).await {
        Some(t) => t,
        None => return Output::Failure(UmtError::RoleNotFound),
    };

    let mut n_role_items = match shared.apps.get_by_id(&a_role.app).await {
        Some(t) => t.default_role,
        None => return Output::Failure(UmtError::AppNotFound),
    };

    a_role.items.merge(&mut n_role_items);
    a_role.items = n_role_items;

    match shared.dao.update_role_by_id(id.as_str(), &a_role).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}
