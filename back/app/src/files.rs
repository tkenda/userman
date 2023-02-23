use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[cfg_attr(target_os = "linux", folder = "../../front/dist/")]
#[cfg_attr(target_os = "macos", folder = "../../front/dist/")]
#[cfg_attr(target_os = "windows", folder = "..\\..\\front\\dist\\")]
struct Asset;

pub struct StaticFile {
    pub uri: Uri,
    pub public_url: String,
}

impl StaticFile {
    pub fn new(uri: Uri, public_url: String) -> Self {
        Self { uri, public_url }
    }
}

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        let trimmed = if self.public_url.is_empty() {
            "/".to_string()
        } else {
            format!("{}/", self.public_url)
        };

        let path = self.uri.path().trim_start_matches(&trimmed).to_string();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                let dst = match (mime.type_().as_str(), mime.subtype().as_str()) {
                    ("text", "html") | ("text", "css") | ("application", "javascript") => {
                        let src = std::str::from_utf8(&content.data).unwrap();
                        Some(src.replace("___PUBLIC_URL___", &self.public_url))
                    }
                    _ => None,
                };

                let body: Full<_> = match dst {
                    Some(t) => t.into(),
                    None => content.data.into(),
                };

                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(boxed(body))
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
