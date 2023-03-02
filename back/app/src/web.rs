use axum::http::Uri;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{v1, openapi::ApiV1Doc};
use crate::{files, Result, Shared, UsermanError};

async fn index_handler(Extension(shared): Extension<Shared>) -> impl IntoResponse {
    let uri = format!("{}/index.html", shared.config_yaml.front.public_url)
        .parse::<Uri>()
        .unwrap();
    files::StaticFile::new(uri, shared.config_yaml.front.public_url)
}

async fn static_handler(Extension(shared): Extension<Shared>, uri: Uri) -> impl IntoResponse {
    files::StaticFile::new(uri, shared.config_yaml.front.public_url)
}

pub async fn run(shared: Shared) -> Result<()> {
    let config_yaml = shared.config_yaml.clone();
    let address = SocketAddr::new(config_yaml.ip, config_yaml.port);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/ApiV1.json", ApiV1Doc::openapi()))
        .route(
            &format!("{}/", config_yaml.front.public_url),
            get(index_handler),
        )
        .route(
            &format!("{}/login", config_yaml.front.public_url),
            get(index_handler),
        )
        .route(
            &format!("{}/users", config_yaml.front.public_url),
            get(index_handler),
        )
        .route(
            &format!("{}/roles", config_yaml.front.public_url),
            get(index_handler),
        )
        .route(
            &format!("{}/apps", config_yaml.front.public_url),
            get(index_handler),
        )
        .nest(
            &format!("{}/api/v1", config_yaml.front.public_url),
            v1::routes(),
        )
        .fallback(static_handler)
        .layer(Extension(shared));

    match config_yaml.tls.enabled {
        true => {
            let rustls = RustlsConfig::from_pem_file(&config_yaml.tls.certs, &config_yaml.tls.key)
                .await
                .map_err(|err| UsermanError::PEMFile(err.to_string()))?;

            axum_server::bind_rustls(address, rustls)
                .serve(app.into_make_service())
                .await
                .map_err(|err| UsermanError::WebServer(err.to_string()))
        }
        false => axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .map_err(|err| UsermanError::WebServer(err.to_string())),
    }
}
