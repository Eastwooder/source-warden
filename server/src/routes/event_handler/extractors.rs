use async_trait::async_trait;
use axum::{
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, HeaderName},
};
use axum_core::{
    extract::Request,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use hyper::StatusCode;
use octocrab::models::webhook_events::WebhookEvent;
use orion::hazardous::mac::hmac::sha256::{SecretKey, Tag};
use thiserror::Error;

use super::ConfigState;

pub struct ExtractSignatureHeader(pub(crate) Sha256VerificationSignature);

#[derive(Clone)]
pub(crate) struct Sha256VerificationSignature(Vec<u8>);

impl<'a> TryFrom<(&'a str, &'a str)> for Sha256VerificationSignature {
    type Error = (StatusCode, &'static str);

    fn try_from((kind, hmac): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        match kind {
            "sha256" => Ok(Sha256VerificationSignature(hex::decode(hmac).map_err(
                |_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to decode"),
            )?)),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported signature type",
            )),
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
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        static HEADER: HeaderName = HeaderName::from_static("x-hub-signature-256");
        if let Some(signature) = parts.headers.get(&HEADER) {
            let (kind, hmac) = signature
                .to_str()
                .map_err(|_| (StatusCode::BAD_REQUEST, "not a valid signature"))?
                .split_once('=')
                .ok_or((StatusCode::BAD_REQUEST, "not a valid signature pair"))?;
            hex::decode(hmac).map_err(|_| (StatusCode::BAD_REQUEST, "not a valid hmac"))?;
            Ok(ExtractSignatureHeader((kind, hmac).try_into()?))
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                "Signature verification header is missing",
            ))
        }
    }
}

pub(crate) struct ExtractEventKind(pub(crate) WebhookEvent);

#[async_trait]
impl FromRequest<ConfigState> for ExtractEventKind {
    type Rejection = (StatusCode, &'static str);

    async fn from_request(
        request: Request,
        ConfigState { webhook_secret }: &ConfigState,
    ) -> Result<Self, Self::Rejection> {
        static HEADER: HeaderName = HeaderName::from_static("x-github-event");
        let (mut parts, body) = request.into_parts();
        let Some(request_kind) = parts.headers.get(&HEADER) else {
            return Err((StatusCode::BAD_REQUEST, "github event header is missing"));
        };
        let kind = request_kind
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "github event header is not valid UTF-8",
                )
            })?
            .to_owned();

        let ExtractSignatureHeader(signature) =
            ExtractSignatureHeader::from_request_parts(&mut parts, &()).await?;

        let body = body
            .collect()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "can not read body"))?
            .to_bytes();

        verify_signature(&signature, webhook_secret, &body)
            .map_err(|_| (StatusCode::BAD_REQUEST, "signature validation failed"))?;

        let event = WebhookEvent::try_from_header_and_body(&kind, &body).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "request is not a valid github event",
            )
        })?;

        Ok(Self(event))
    }
}

fn verify_signature(
    signature: &Sha256VerificationSignature,
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
}

impl IntoResponse for VerificationError {
    fn into_response(self) -> Response {
        match self {
            VerificationError::InvalidSignature => {
                (StatusCode::BAD_REQUEST, "Invalid Signature").into_response()
            }
            VerificationError::SignatureMismatch => {
                (StatusCode::BAD_REQUEST, "Signature mismatch").into_response()
            }
        }
    }
}
