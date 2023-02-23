use log::error;
use tokio::sync::mpsc;

use crate::dao::Memory;

use crate::{Result, Shared};

#[derive(Debug)]
pub enum Event {
    Configs,
    Users,
    Roles,
    Apps,
}

pub async fn run(shared: Shared) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<Event>(100);

    let tx_ref = tx.clone();
    let dao_ref = shared.dao.clone();

    tokio::spawn(async move {
        if let Err(err) = dao_ref.watch_configs(&tx_ref).await {
            error!("{}", err);
        }
    });

    let tx_ref = tx.clone();
    let dao_ref = shared.dao.clone();

    tokio::spawn(async move {
        if let Err(err) = dao_ref.watch_users(&tx_ref).await {
            error!("{}", err);
        }
    });

    let tx_ref = tx.clone();
    let dao_ref = shared.dao.clone();

    tokio::spawn(async move {
        if let Err(err) = dao_ref.watch_roles(&tx_ref).await {
            error!("{}", err);
        }
    });

    let dao_ref = shared.dao.clone();

    tokio::spawn(async move {
        if let Err(err) = dao_ref.watch_apps(&tx).await {
            error!("{}", err);
        }
    });

    while let Some(event) = rx.recv().await {
        match event {
            Event::Configs => {
                if let Err(err) = shared.configs.reload(&shared.dao).await {
                    error!("{}", err);
                }

                if let Err(err) = shared.keys.reload(&shared.dao).await {
                    error!("{}", err);
                }
            }
            Event::Users => {
                if let Err(err) = shared.users.reload(&shared.dao).await {
                    error!("{}", err);
                }
            }
            Event::Roles => {
                if let Err(err) = shared.roles.reload(&shared.dao).await {
                    error!("{}", err);
                }
            }
            Event::Apps => {
                if let Err(err) = shared.apps.reload(&shared.dao).await {
                    error!("{}", err);
                }
            }
        }
    }

    Ok(())
}
