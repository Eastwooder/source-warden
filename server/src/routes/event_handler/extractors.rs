use async_trait::async_trait;
use axum::{
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, HeaderName},
};
use axum_core::{
    extract::Request,
    response::{IntoResponse, Response},
};
use hex::FromHexError;
use http_body_util::BodyExt;
use hyper::{header::ToStrError, StatusCode};
use octocrab::models::webhook_events::WebhookEvent;
use orion::hazardous::mac::hmac::sha256::{SecretKey, Tag};
use thiserror::Error;

use super::ConfigState;

pub struct ExtractSignatureHeader(pub(crate) Sha256VerificationSignature);

#[derive(Clone)]
pub(crate) struct Sha256VerificationSignature(Vec<u8>);

impl<'a> TryFrom<(&'a str, &'a str)> for Sha256VerificationSignature {
    type Error = SignatureHeaderError;

    fn try_from((kind, hmac): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        match kind {
            "sha256" => Ok(Sha256VerificationSignature(hex::decode(hmac)?)),
            _ => Err(SignatureHeaderError::MissingHeader),
        }
    }
}

impl PartialEq<Tag> for &Sha256VerificationSignature {
    fn eq(&self, other: &Tag) -> bool {
        *other == &*self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractSignatureHeader
where
    S: Send + Sync,
{
    type Rejection = SignatureHeaderError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        static HEADER: HeaderName = HeaderName::from_static("x-hub-signature-256");
        let Some(signature) = parts.headers.get(&HEADER) else {
            return Err(SignatureHeaderError::MissingHeader);
        };
        let (kind, hmac) = signature
            .to_str()?
            .split_once('=')
            .ok_or(SignatureHeaderError::NotAPair)?;
        Ok(ExtractSignatureHeader((kind, hmac).try_into()?))
    }
}

#[derive(Debug, Error)]
pub enum SignatureHeaderError {
    #[error("The header value does not consist of a valid string")]
    InvalidValue(#[from] ToStrError),
    #[error("The header value is missing the correct delimiter")]
    NotAPair,
    #[error("The header value is not a valid hex value")]
    NotHex(#[from] FromHexError),
    #[error("Missing header pair (either left or right side)")]
    MissingHeader,
}

impl IntoResponse for SignatureHeaderError {
    fn into_response(self) -> Response {
        match self {
            e @ SignatureHeaderError::InvalidValue(_) => (StatusCode::BAD_REQUEST, e.to_string()),
            e @ SignatureHeaderError::NotAPair => (StatusCode::BAD_REQUEST, e.to_string()),
            e @ SignatureHeaderError::NotHex(_) => (StatusCode::BAD_REQUEST, e.to_string()),
            e @ SignatureHeaderError::MissingHeader => (StatusCode::BAD_REQUEST, e.to_string()),
        }
        .into_response()
    }
}

pub(crate) struct GitHubEvent(pub(crate) WebhookEvent);

#[async_trait]
impl FromRequest<ConfigState> for GitHubEvent {
    type Rejection = GitHubEventExtractionError;

    async fn from_request(
        request: Request,
        ConfigState { webhook_secret }: &ConfigState,
    ) -> Result<Self, Self::Rejection> {
        static HEADER: HeaderName = HeaderName::from_static("x-github-event");
        let (mut parts, body) = request.into_parts();
        let Some(request_kind) = parts.headers.get(&HEADER) else {
            return Err(GitHubEventExtractionError::MissingEventHeader);
        };
        let kind = request_kind.to_str()?.to_owned();

        let ExtractSignatureHeader(signature) =
            ExtractSignatureHeader::from_request_parts(&mut parts, &()).await?;

        let body = body.collect().await?.to_bytes();

        verify_signature(&signature, webhook_secret, &body)?;

        Ok(Self(
            WebhookEvent::try_from_header_and_body(&kind, &body)
                .map_err(GitHubEventExtractionError::EventUnparsable)?,
        ))
    }
}

fn verify_signature(
    signature: &Sha256VerificationSignature,
    webhook_secret: &SecretKey,
    body: &[u8],
) -> Result<(), GitHubEventExtractionError> {
    use orion::hazardous::mac::hmac::sha256::HmacSha256;
    let tag = HmacSha256::hmac(webhook_secret, body)
        .map_err(|_| GitHubEventExtractionError::InvalidSignature)?;
    if signature == tag {
        Ok(())
    } else {
        Err(GitHubEventExtractionError::SignatureMismatch)
    }
}

#[derive(Debug, Error)]
pub enum GitHubEventExtractionError {
    #[error("Missing event header")]
    MissingEventHeader,
    #[error("The header value does not consist of a valid string")]
    InvalidValue(#[from] ToStrError),
    #[error("Unable to calculate the signature of the body")]
    InvalidSignature,
    #[error("Signature of body does not match the header")]
    SignatureMismatch,
    #[error("Unable to verify the signature: {0}")]
    SignatureError(#[from] SignatureHeaderError),
    #[error("Unable to parse and process the request")]
    EventUnparsable(serde_json::Error),
    #[error("Something went wrong whilst processing the body")]
    AxumError(#[from] axum::Error),
}

impl IntoResponse for GitHubEventExtractionError {
    fn into_response(self) -> Response {
        match self {
            e @ GitHubEventExtractionError::MissingEventHeader => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::InvalidValue(_) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::InvalidSignature => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::SignatureMismatch => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::SignatureError(_) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::EventUnparsable(_) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            e @ GitHubEventExtractionError::AxumError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
        .into_response()
    }
}
