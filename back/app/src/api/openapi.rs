use userman_auth::apps::{App, AppsVec};
use userman_auth::roles::{
    DataOptions, DataValue, Item, Role, RoleItems, RoleValues, RolesVec, Value,
};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::users::{User, UsersVec};

use super::v1;

#[derive(OpenApi)]
#[openapi(
    paths(
        v1::sessions::login,
        v1::sessions::refresh,
        v1::sessions::logout,
        v1::sessions::refresh,
        v1::apps::create,
        v1::apps::read,
        v1::apps::read_all,
        v1::apps::update,
        v1::apps::delete,
        v1::roles::create,
        v1::roles::read,
        v1::roles::read_all,
        v1::roles::read_all_names,
        v1::roles::update,
        v1::roles::delete,
        v1::roles::name,
        v1::roles::sync,
        v1::users::create,
        v1::users::read,
        v1::users::read_all,
        v1::users::update,
        v1::users::reset,
        v1::users::delete,
        v1::users::username,
    ),
    components(
        schemas(
            App,
            AppsVec,
            User,
            UsersVec,
            Item,
            Value,
            Role,
            RolesVec,
            RoleItems,
            RoleValues,
            DataValue,
            DataOptions,
            v1::StringsVec,
            v1::StatusGeneric,
            v1::sessions::LoginReq,
            v1::sessions::LoginRes,
            v1::StatusLoginRes,
            v1::sessions::RefreshReq,
            v1::sessions::RefreshRes,
            v1::StatusRefreshRes,
            v1::sessions::LogoutReq,
            v1::sessions::ResetReq,
            v1::StatusApp,
            v1::StatusApps,
            v1::StatusRole,
            v1::StatusRoles,
            v1::StatusStrings,
            v1::StatusUser,
            v1::StatusUsers,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "API V1")
    )
)]
pub struct ApiV1Doc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}
