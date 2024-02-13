use async_trait::async_trait;
use axum::{
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, HeaderName},
};
use axum_core::extract::Request;
use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::{header::CONTENT_LENGTH, StatusCode};
use octocrab::models::webhook_events::WebhookEvent;
use orion::hazardous::mac::hmac::sha256::Tag;

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

impl PartialEq<Tag> for Sha256VerificationSignature {
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
        const HEADER: HeaderName = HeaderName::from_static("x-hub-signature-256");
        if let Some(signature) = parts.headers.get(HEADER) {
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

pub struct ExtractContentLength(pub(crate) usize);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractContentLength
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(content_length) = parts.headers.get(CONTENT_LENGTH) else {
            return Err((StatusCode::BAD_REQUEST, "content length is missing!"));
        };
        let content_length: usize = content_length
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "content length is not UTF-8"))?
            .parse()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "content length is not a positive number",
                )
            })?;
        Ok(Self(content_length))
    }
}

pub(crate) struct ExtractEventKind(pub(crate) WebhookEvent);

#[async_trait]
impl<S> FromRequest<S> for ExtractEventKind
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
        const HEADER: HeaderName = HeaderName::from_static("x-github-event");
        let (parts, body) = request.into_parts();
        let Some(request_kind) = parts.headers.get(HEADER) else {
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

        let body = body
            .collect()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "can not read body"))?
            .to_bytes();
        let event = WebhookEvent::try_from_header_and_body(&kind, &body).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "request is not a valid github event",
            )
        })?;

        Ok(Self(event))
    }
}
