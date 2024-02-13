use jsonwebtoken::EncodingKey;
use octocrab::{models::AppId, Octocrab};
use thiserror::Error;

pub fn authenticate(
    app_id: AppId,
    app_key: EncodingKey,
) -> Result<ApplicationAuthentication, AuthenticationError> {
    let client = Octocrab::builder().app(app_id, app_key).build()?;
    Ok(ApplicationAuthentication { client })
}

#[derive(Clone)]
pub struct ApplicationAuthentication {
    pub client: Octocrab,
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Error whilst creating the authentication: {0}")]
    Octocrab(#[from] octocrab::Error),
}
