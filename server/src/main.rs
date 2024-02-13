use axum::Router;
use octocrab::models::AppId;
use server::config::{load_github_app_config, GitHubAppConfiguration};
use server::routes;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing()?;

    use jsonwebtoken::EncodingKey;
    use orion::hazardous::mac::hmac::sha256::SecretKey as Sha256SecretKey;
    let app_config = load_github_app_config().unwrap_or(GitHubAppConfiguration {
        webhook_secret: Sha256SecretKey::generate().into(),
        app_identifier: AppId(1),
        app_key: {
            use rand::SeedableRng;
            use rsa::pkcs8::EncodePrivateKey;
            // let mut rng = rand::thread_rng();
            let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(17_832_551);
            let bits = 2048;
            let priv_key =
                rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            let _pub_key = rsa::RsaPublicKey::from(&priv_key);

            let der_encoded_key = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
            let cert_pem_str = der_encoded_key.to_string();
            println!("{cert_pem_str}");

            EncodingKey::from_rsa_pem(cert_pem_str.as_bytes()).unwrap()
        },
    });
    // let app_config = config::load_github_app_config()?;

    let app = Router::new()
        .merge(routes::ui::router(&app_config))
        .merge(routes::event_handler::router(&app_config));

    let listener = {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        tracing::debug!("going to listen on {}", addr);
        TcpListener::bind(addr).await?
    };
    axum::serve(listener, app).await?;

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
