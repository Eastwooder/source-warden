use std::{pin::pin, sync::Arc};

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};

use bytes::Bytes;
use futures_util::stream::StreamExt;
use orion::hazardous::mac::hmac::sha256::SecretKey;
use thiserror::Error;

use crate::config::GitHubAppConfiguration;

use self::extractors::{ExtractContentLength, ExtractSignatureHeader, Sha256VerificationSignature};

mod extractors;

pub fn router(config: &GitHubAppConfiguration) -> Router {
    let signature_config = ConfigState {
        webhook_secret: config.webhook_secret.clone(),
    };
    let request_verification_layer = from_fn_with_state(signature_config, ensure_payload_is_valid);
    Router::new().route(
        "/event_handler",
        any(handle_github_event).layer(request_verification_layer),
    )
}

#[derive(Clone)]
struct ConfigState {
    webhook_secret: Arc<SecretKey>,
}

async fn handle_github_event(body: String) -> impl IntoResponse {
    tracing::error!(len = body.len(), body = body, "logic starts now");
    "hello world"
}

async fn ensure_payload_is_valid(
    State(ConfigState { webhook_secret }): State<ConfigState>,
    ExtractSignatureHeader(signature): ExtractSignatureHeader,
    ExtractContentLength(content_length): ExtractContentLength,
    request: Request,
    next: Next,
) -> Result<Response, VerificationError> {
    tracing::error!("we're here");
    let (request, body) = split_request(content_length, request).await?;

    verify_signature(signature, &webhook_secret, &body)?;

    Ok(next.run(request).await)
}

async fn split_request(
    content_length: usize,
    request: Request,
) -> Result<(Request, Bytes), VerificationError> {
    let (parts, body) = request.into_parts();

    let body_stream = body.into_data_stream();
    let max_peek_bytes = content_length;
    let mut body_buffer = bytes::BytesMut::with_capacity(max_peek_bytes);
    {
        let mut body_stream = body_stream.peekable();
        let mut body_stream = pin!(body_stream);
        while body_buffer.len() < max_peek_bytes {
            match body_stream.as_mut().peek().await {
                Some(Ok(chunk)) => body_buffer.extend_from_slice(chunk),
                Some(Err(_)) => return Err(VerificationError::CannotReadBody),
                None => break, // End of body
            }
        }
    }
    Ok((
        Request::from_parts(parts, Body::from(body_buffer.to_vec())),
        body_buffer.into(),
    ))
}

fn verify_signature(
    signature: Sha256VerificationSignature,
    webhook_secret: &SecretKey,
    body: &[u8],
) -> Result<(), VerificationError> {
    use orion::hazardous::mac::hmac::sha256::HmacSha256;
    let tag =
        HmacSha256::hmac(webhook_secret, body).map_err(|_| VerificationError::InvalidSignature)?;
    if signature == tag {
        Ok(())
    } else {
        Err(VerificationError::SignatureMismatch)
    }
}

#[derive(Debug, Error)]
enum VerificationError {
    #[error("Unable to calculate the signature of the body")]
    InvalidSignature,
    #[error("Signature of body does not match the header")]
    SignatureMismatch,
    #[error("Unable to read the body to the end")]
    CannotReadBody,
}

impl IntoResponse for VerificationError {
    fn into_response(self) -> Response {
        match self {
            VerificationError::InvalidSignature => {
                (StatusCode::BAD_REQUEST, "Invalid Signature").into_response()
            }
            VerificationError::CannotReadBody => {
                (StatusCode::BAD_REQUEST, "Body Length mismatch").into_response()
            }
            VerificationError::SignatureMismatch => {
                (StatusCode::BAD_REQUEST, "Signature mismatch").into_response()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use hyper::StatusCode;
    use orion::hazardous::mac::hmac::sha256::{HmacSha256, SecretKey};
    use rsa::RsaPublicKey;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::config::GitHubAppConfiguration;

    #[tracing_test::traced_test]
    #[tokio::test]
    async fn test_happy_path() {
        tracing::info!("what");
        let (config, _) = create_test_config();
        let app = super::router(&config);

        let body = serde_json::to_vec(&json!({"hello": "world"})).unwrap();
        let body_hmac = calc_hmac_for_body(&config.webhook_secret, &body);
        let request = Request::builder()
            .uri("/event_handler")
            .header("x-hub-signature-256", format!("sha256={body_hmac}"))
            .header("Content-Length", body.len())
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        // let body: serde_json::Value = str::get(&body).unwrap();
        tracing::info!(?body);
    }

    #[tracing_test::traced_test]
    #[tokio::test]
    async fn test_missing_signature() {
        let (config, _) = create_test_config();
        let app = super::router(&config);

        let body = serde_json::to_vec(&json!({"hello": "world"})).unwrap();
        let request = Request::builder()
            .uri("/event_handler")
            .header("Content-Length", body.len())
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tracing_test::traced_test]
    #[tokio::test]
    async fn test_wrong_signature() {
        let (config, _) = create_test_config();
        let app = super::router(&config);

        let body = serde_json::to_vec(&json!({"hello": "world"})).unwrap();
        let request = Request::builder()
            .uri("/event_handler")
            .header(
                "x-hub-signature-256",
                "sha256=46288437613044114D21E7FAD79837C12336202F4C85008548FB226693426F56",
            )
            .header("Content-Length", body.len())
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    fn create_test_config() -> (GitHubAppConfiguration, RsaPublicKey) {
        use jsonwebtoken::EncodingKey;
        use octocrab::models::AppId;
        use rand::SeedableRng;
        use rsa::pkcs8::EncodePrivateKey;

        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(17_832_551);
        let bits = 256;
        let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = rsa::RsaPublicKey::from(&priv_key);

        let der_encoded_key = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let cert_pem_str = der_encoded_key.to_string();

        let secret = SecretKey::from_slice(&[0; 32]).unwrap();

        (
            GitHubAppConfiguration {
                webhook_secret: secret.into(),
                app_identifier: AppId(1),
                app_key: { EncodingKey::from_rsa_pem(cert_pem_str.as_bytes()).unwrap() },
            },
            pub_key,
        )
    }

    fn calc_hmac_for_body(secret: &SecretKey, data: &[u8]) -> String {
        hex::encode(
            HmacSha256::hmac(&secret, data)
                .unwrap()
                .unprotected_as_bytes(),
        )
    }
}
