#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::routing::get;
    use axum::Router;
    use std::net::SocketAddr;

    setup_tracing()?;

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub(crate) fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()?;

    Ok(())
}

#[cfg(debug_assertions)]
async fn root() -> String {
    let serve = std::env::var("CLIENT_DIST").unwrap();
    format!("Hello {serve}")
}

#[cfg(not(debug_assertions))]
async fn root() -> &'static str {
    const SERVE: &str = env!("CLIENT_DIST");
    const_format::concatcp!("Hello ", SERVE)
}
