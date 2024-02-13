use axum::{routing::get, Router};

use crate::config::GitHubAppConfiguration;

pub fn router(_: &GitHubAppConfiguration) -> Router {
    Router::new().route("/ui", get(frontend_ui))
}

#[cfg(debug_assertions)]
async fn frontend_ui() -> String {
    let serve = std::env::var("CLIENT_DIST").unwrap();
    format!("Hello {serve}")
}

#[cfg(not(debug_assertions))]
async fn frontend_ui() -> &'static str {
    const SERVE: &str = env!("CLIENT_DIST");
    const_format::concatcp!("Hello ", SERVE)
}
