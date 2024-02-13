pub mod config;
pub mod routes;

use std::net::SocketAddr;

use axum::{middleware::from_fn, Router};
use config::GitHubAppConfiguration;
pub use routes::metrics::track_metrics;
use tokio::net::TcpListener;

pub async fn main_app(app_config: GitHubAppConfiguration) -> Result<(), std::io::Error> {
    let main_app = Router::new()
        .merge(routes::ui::router(&app_config))
        .merge(routes::event_handler::router(&app_config))
        .route_layer(from_fn(track_metrics));

    let main_listener = {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        tracing::debug!("going to listen on {}", addr);
        TcpListener::bind(addr).await?
    };

    axum::serve(main_listener, main_app).await
}

pub async fn metrics_app() -> Result<(), std::io::Error> {
    let metrics_app = routes::metrics::router();
    let metrics_listener = {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
        tracing::debug!("going to listen on {}", addr);
        TcpListener::bind(addr).await?
    };

    axum::serve(metrics_listener, metrics_app).await
}
