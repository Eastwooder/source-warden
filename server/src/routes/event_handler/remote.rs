use jsonwebtoken::EncodingKey;
use octocrab::models::AppId;

pub fn authenticate(_app_id: AppId, _app_key: EncodingKey) -> ApplicationAuthentication {
    ApplicationAuthentication {}
}

#[derive(Clone)]
pub struct ApplicationAuthentication {
    // pub client: Octocrab,
}
