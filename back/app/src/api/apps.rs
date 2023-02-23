use std::str::FromStr;

use axum::extract::{Json, Path};
use axum::response::{Extension, IntoResponse};
use mongodb::bson::oid::ObjectId;

use userman_auth::app::App;

use crate::dao::Memory;
use crate::error::UmtError;
use crate::tokens::SessionToken;
use crate::Shared;

use super::Output;

pub(crate) async fn create(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<App>,
) -> impl IntoResponse {
    let apps = roles!(shared, token);
    let create = value!(apps, "/apps/create.boolean");

    validate_bool!(create);

    match shared.dao.create_app(&payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn read(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let apps = roles!(shared, token);
    let read = value!(apps, "/apps/read.boolean");

    validate_bool!(read);

    let object_id = match ObjectId::from_str(id.as_str()).map_err(UmtError::ParseObjectId) {
        Ok(t) => t,
        Err(err) => return Output::Failure(err),
    };

    match shared.apps.get_by_id(&object_id).await {
        Some(t) => Output::Success(t),
        None => Output::Done,
    }
}

pub(crate) async fn read_all(
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let apps = roles!(shared, token);
    let read = value!(apps, "/apps/read.boolean");

    validate_bool!(read);

    let values = shared.apps.get_all().await;
    Output::Success(values)
}

pub(crate) async fn update(
    id: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
    Json(payload): Json<App>,
) -> impl IntoResponse {
    let apps = roles!(shared, token);
    let update = value!(apps, "/apps/update.boolean");

    validate_bool!(update);

    match shared.dao.update_app_by_id(id.as_str(), &payload).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}

pub(crate) async fn delete(
    app: Path<String>,
    token: SessionToken,
    Extension(shared): Extension<Shared>,
) -> impl IntoResponse {
    let apps = roles!(shared, token);
    let delete = value!(apps, "/apps/delete.boolean");

    validate_bool!(delete);

    match shared.dao.delete_app_by_id(app.to_string()).await {
        Ok(_) => Output::<()>::Done,
        Err(err) => Output::Failure(err),
    }
}
