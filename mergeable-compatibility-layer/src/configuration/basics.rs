use std::fmt::Debug;

use crate::configuration::options::{BeginsWith, EndsWith, MustExclude, MustInclude};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T: Debug> {
    Many(Vec<T>),
    Single(T),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageClause(String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum MatchClause {
    Long {
        #[serde(rename = "match")]
        match_clause: OneOrMany<String>,
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<MessageClause>,
    },
    Short(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum RegexClause {
    Long {
        #[serde(rename = "regex")]
        regex: OneOrMany<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "regex_flag")]
        regex_flag: Option<String>,
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<MessageClause>,
    },
    Short(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CountClause {
    #[serde(rename = "count")]
    count: u64,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeClause {
    #[serde(rename = "days")]
    days: u32,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BooleanClause {
    #[serde(rename = "match")]
    match_clause: bool,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChainedAndOrIncludeExcludeClause {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Self>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<Self>>,
    #[serde(rename = "must_include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<MustInclude>,
    #[serde(rename = "must_exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<MustExclude>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChainedAndOrIncludeExcludeClauseBeginsEnds {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Self>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<Self>>,
    #[serde(rename = "must_include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<MustInclude>,
    #[serde(rename = "must_exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<MustExclude>,
    #[serde(rename = "begins_with")]
    #[serde(skip_serializing_if = "Option::is_none")]
    begins_with: Option<BeginsWith>,
    #[serde(rename = "ends_with")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ends_with: Option<EndsWith>,
}
