use octocrab::Octocrab;
pub trait GitHubActionalbe {
    fn post_message(&self);
}

impl GitHubActionalbe for Octocrab {
    fn post_message(&self) {
        todo!()
    }
}
