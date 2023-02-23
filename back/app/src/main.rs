mod api;
mod apps;
mod config_yaml;
mod configs;
mod dao;
mod error;
mod logger;
mod roles;
mod tokens;
mod users;
mod watchers;

#[cfg(test)]
mod tests;

use userman_auth::role::RoleItems;
use userman_auth::app::LOCAL_APP;
use userman_auth::Auth;
use config_yaml::ConfigYAML;
use configs::Configs;
use dao::{Dao, Memory};
use error::UmtError;
use logger::LogsLevel;
use mongodb::bson::oid::ObjectId;
use serde::ser::SerializeSeq;
use tokens::{Keys, SessionToken};

pub type Result<T> = std::result::Result<T, UmtError>;

const YAML_FILE: &str = "config.yaml";
const VERSION: &str = env!("CARGO_PKG_VERSION");

use apps::Apps;
use log::{error, info};
use roles::Roles;
use users::Users;

pub fn serialize_option_oid_as_string<S>(
    oid: &Option<ObjectId>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match oid {
        Some(ref t) => serializer.serialize_some(t.to_string().as_str()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_vec_oid_as_string<S>(
    oid: &Vec<ObjectId>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(Some(oid.len()))?;

    for element in oid {
        seq.serialize_element(element.to_string().as_str())?;
    }

    seq.end()
}

pub fn serialize_oid_as_string<S>(
    oid: &ObjectId,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(oid.to_string().as_str())
}

/// Wait for all threads to finish for a graceful shutdown.
#[cfg(unix)]
pub(crate) async fn wait_for_shutdown() -> Result<()> {
    use tokio::signal::unix::{signal, SignalKind};

    async fn interrupt() -> Result<()> {
        let mut sigint = signal(SignalKind::interrupt()).map_err(UmtError::SignalInterrupt)?;

        sigint.recv().await.ok_or(UmtError::InterruptBlocking)?;

        info!("Main thread received SIGINT.");
        Ok(())
    }

    async fn terminate() -> Result<()> {
        let mut sigterm = signal(SignalKind::terminate()).map_err(UmtError::SignalTerminate)?;

        sigterm.recv().await.ok_or(UmtError::TerminateBlocking)?;

        info!("Main thread received SIGTERM.");
        Ok(())
    }

    tokio::select! {
        resint = interrupt() => resint,
        resterm = terminate() => resterm,
    }
}

/// Wait for all threads to finish for a graceful shutdown.
#[cfg(windows)]
pub(crate) async fn wait_for_shutdown() -> Result<()> {
    use tokio::signal;

    signal::ctrl_c()
        .await
        .map_err(|_| ViewerError::InterruptBlocking)?;

    info!("Main thread received SIGINT.");
    Ok(())
}

#[derive(Clone)]
pub struct Shared {
    auth: Auth,
    dao: Dao,
    config_yaml: ConfigYAML,
    configs: Configs,
    apps: Apps,
    keys: Keys,
    users: Users,
    roles: Roles,
}

impl Shared {
    async fn role_items(&self, token: SessionToken) -> Result<RoleItems> {
        let role_names = token.role_names(&self.keys).await?;
        Ok(self.auth.add_role_items(role_names).await)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut logger = logger::build();

    info!("Proteus UMT v{}", VERSION);

    let config_yaml = ConfigYAML::read_or_create_file(YAML_FILE).await?;

    if config_yaml.logs != LogsLevel::Info {
        logger.set_logger(&config_yaml.logs);
        info!("Logger in {} mode.", &config_yaml.logs);
    }

    let dao = config_yaml.dao().await?;
    dao.init().await?;

    let auth = Auth::builder(LOCAL_APP).build().await?;
    auth.init().await?;

    let shared = Shared {
        configs: Configs::load(&dao).await?,
        apps: Apps::load(&dao).await?,
        keys: Keys::load(&dao).await?,
        users: Users::load(&dao).await?,
        roles: Roles::load(&dao).await?,
        config_yaml,
        auth,
        dao,
    };

    let shared_ref = shared.clone();

    tokio::spawn(async move {
        if let Err(err) = api::run(shared_ref).await {
            error!("{}", err);
        }
    });

    tokio::spawn(async move {
        if let Err(err) = watchers::run(shared).await {
            error!("{}", err);
        }
    });

    wait_for_shutdown().await
}
