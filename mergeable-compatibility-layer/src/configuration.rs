use serde::{Deserialize, Serialize};

pub mod actions;
pub mod basics;
pub mod error;
pub mod fail;
pub mod filter;
pub mod options;
pub mod pass;
pub mod validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    version: u32,
    mergeable: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    when: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<filter::Filter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<Vec<validate::Validator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pass: Option<Vec<pass::Pass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail: Option<Vec<fail::Fail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Vec<error::Error>>,
}
