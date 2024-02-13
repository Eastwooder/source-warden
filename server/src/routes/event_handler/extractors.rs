use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderName},
};
use hyper::{header::CONTENT_LENGTH, StatusCode};
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
