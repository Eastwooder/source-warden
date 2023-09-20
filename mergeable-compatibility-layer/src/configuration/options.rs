use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::configuration::basics::{
    BooleanClause, CountClause, MatchClause, MessageClause, OneOrMany, RegexClause,
};

/// Supported Validators:
///   'payload'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommonBoolean(BooleanClause);

/// Supported Validators:
///   'changeset', 'content', 'description', 'label', 'milestone', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BeginsWith(MatchClause);

/// Supported Validators:
///   'changeset', 'content', 'description', 'label', 'milestone', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EndsWith(MatchClause);

/// Supported Validators:
///   'baseRef', 'headRef', 'changeset', 'commit', 'content', 'description', 'label', 'milestone', 'project', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MustInclude(RegexClause);

/// Supported Validators:
///   'baseRef', 'headRef', 'changeset', 'content', 'description', 'label', 'milestone', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MustExclude(RegexClause);

/// Supported Validators:
///   'changeset', 'description', 'label', 'milestone', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoEmpty {
    #[serde(rename = "enabled")]
    enabled: bool,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

/// Supported Validators:
///   'approvals'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Required {
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<OneOrMany<String>>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    owners: Option<bool>,
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<bool>,
    #[serde(rename = "requested_reviewers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_reviewers: Option<bool>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

/// Supported Validators:
///   'approvals', 'assignee', 'changeset', 'label', 'size'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Min(CountClause);

/// Supported Validators:
///   'approvals', 'assignee', 'changeset', 'label'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Max(CountClause);

/// Supported Validators:
///   'commit', 'description', 'headRef', 'label', 'milestone', 'title'
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Jira(RegexClause);
