macro_rules! permissions {
    ($shared:ident, $token:ident) => {
        match $shared.permissions($token).await {
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
        use userman_auth::roles::DataValue;

        match $value {
            DataValue::Boolean(true) => {}
            _ => return Output::Unauthorized(UsermanError::Unauthorized),
        }
    };
}

pub mod openapi;
pub mod v1;
