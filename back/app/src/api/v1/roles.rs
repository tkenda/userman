use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use userman_auth::roles::{Role, RolesVec};

use super::{Output, Example, Status, StringsVec};
use crate::dao::Memory;
use crate::error::UsermanError;
use crate::roles::RoleName;
use crate::tokens::SessionToken;
use crate::Shared;

impl Example for Role {
    fn example() -> Self {
        Self::default()
    }
}

impl Example for RolesVec {
    fn example() -> Self {
        Self(vec![Role::default()])
    }
}

impl Example for StringsVec {
    fn example() -> Self {
        Self(vec!["NAME1".to_string(), "NAME2".to_string()])
    }
}

#[utoipa::path(
    post, 
    path = "/api/v1/roles",
    request_body(content = Role, example = json!(Role::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Create role successfully", 
            body = StatusGeneric,
            example = json!(Status::<Role>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Create role with error",
            body = StatusGeneric,
            example = json!(Status::<Role>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<Role>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let create = value!(items, "/roles/create.boolean");

    validate_bool!(create);

    match shared.dao.create_role(&payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/roles/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read role successfully", 
            body = StatusRole,
            example = json!(Status::<Role>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read role with error",
            body = StatusRole,
            example = json!(Status::<Role>::example_bad_request())
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
    let read = value!(items, "/roles/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UsermanError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.roles.get_by_id(&object_id).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/roles",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read roles successfully", 
            body = StatusRoles,
            example = json!(Status::<RolesVec>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read roles with error",
            body = StatusRoles,
            example = json!(Status::<RolesVec>::example_bad_request())
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
    let read = value!(items, "/roles/read.boolean");

    validate_bool!(read);

    let values = shared.roles.get_all().await;
    Output::Success(values)
}

#[utoipa::path(
    get, 
    path = "/api/v1/rolenames",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read role rolenames successfully", 
            body = StatusStrings,
            example = json!(Status::<StringsVec>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read role rolenames with error",
            body = StatusStrings,
            example = json!(Status::<StringsVec>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn read_all_names(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let read = value!(items, "/roles/read.boolean");

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

#[utoipa::path(
    put, 
    path = "/api/v1/roles/<id>",
    request_body(content = Role, example = json!(Role::example())),
    responses(
        (
            status = StatusCode::OK, 
            description = "Update role successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Update role with error",
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
    Json(payload): Json<Role>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let update = value!(items, "/roles/update.boolean");

    validate_bool!(update);

    match shared.dao.update_role_by_id(id.as_str(), &payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    delete, 
    path = "/api/v1/roles/<id>",
    responses(
        (
            status = StatusCode::OK, 
            description = "Delete role successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Delete role with error",
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
    let delete = value!(items, "/roles/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_role_by_id(id.as_str()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/rolenames/:name",
    responses(
        (
            status = StatusCode::OK, 
            description = "Read role by rolename successfully", 
            body = StatusRole,
            example = json!(Status::<Role>::example_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Read role by rolename with error",
            body = StatusRole,
            example = json!(Status::<Role>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn name(
    name: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let read = value!(items, "/roles/read.boolean");

    validate_bool!(read);

    match shared.roles.get(&name).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

#[utoipa::path(
    get, 
    path = "/api/v1/roles/<id>/sync",
    responses(
        (
            status = StatusCode::OK, 
            description = "Sync role successfully", 
            body = StatusGeneric,
            example = json!(Status::<()>::example_empty_ok())
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Sync role with error",
            body = StatusGeneric,
            example = json!(Status::<()>::example_bad_request())
        )
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn sync(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let items = permissions!(shared, token);
    let update = value!(items, "/roles/update.boolean");

    validate_bool!(update);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UsermanError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    let mut a_role = match shared.roles.get_by_id(&object_id).await {
        Some(t) => t,
        None => return Output::Failure(UsermanError::RoleNotFound),
    };

    let mut n_role_items = match shared.apps.get_by_id(&a_role.app).await {
        Some(t) => t.default_role,
        None => return Output::Failure(UsermanError::AppNotFound),
    };

    a_role.items.merge(&mut n_role_items);
    a_role.items = n_role_items;

    match shared.dao.update_role_by_id(id.as_str(), &a_role).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}
