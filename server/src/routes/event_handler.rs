use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, HeaderName, StatusCode},
    middleware::{from_fn, from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use jsonwebtoken::EncodingKey;

use crate::config::GitHubAppConfiguration;

pub fn router(config: &GitHubAppConfiguration) -> Router {
    let signature_verification_state = ConfigState {
        app_key: config.app_key.clone(),
    };
    Router::new()
        .route("/event_handler", any(handle_github_event))
        .layer(from_fn_with_state(
            signature_verification_state,
            ensure_payload_is_valid,
        ))
}

#[derive(Clone)]
struct ConfigState {
    app_key: EncodingKey,
}

async fn handle_github_event() -> impl IntoResponse {
    tracing::error!("logic starts now");
    "hello world"
}

async fn ensure_payload_is_valid(
    State(ConfigState { app_key }): State<ConfigState>,
    ExtractSignatureHeader(_signature): ExtractSignatureHeader,
    request: Request,
    next: Next,
) -> Response {
    tracing::error!("we're here");
    next.run(request).await
}

#[derive(Clone)]
struct ExtractSignatureHeader(VerificationSignature);

#[derive(Clone)]
enum VerificationSignature {
    Sha256(Vec<u8>),
}

impl<'a> TryFrom<(&'a str, &'a str)> for VerificationSignature {
    type Error = (StatusCode, &'static str);

    fn try_from((kind, hmac): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        match kind {
            "sha256" => Ok(VerificationSignature::Sha256(hex::decode(hmac).map_err(
                |_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to decode"),
            )?)),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported signature type",
            )),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractSignatureHeader
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        static HEADER: HeaderName = HeaderName::from_static("x-hub-signature-256");
        if let Some(signature) = parts.headers.get(&HEADER) {
            let (kind, hmac) = signature
                .to_str()
                .map_err(|_| (StatusCode::BAD_REQUEST, "not a valid signature"))?
                .split_once('=')
                .ok_or((StatusCode::BAD_REQUEST, "not a valid signature pair"))?;
            Ok(ExtractSignatureHeader((kind, hmac).try_into()?))
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                "Signature verification header is missing",
            ))
        }
    }
}
