use crate::configuration::basics::{
    ChainedAndOrIncludeExcludeClause, ChainedAndOrIncludeExcludeClauseBeginsEnds, CountClause,
    MessageClause, TimeClause,
};
use serde::{Deserialize, Serialize};

use crate::configuration::options::{
    BeginsWith, EndsWith, Jira, Max, Min, MustExclude, MustInclude, NoEmpty, Required,
};

use super::basics::OneOrMany;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "do")]
#[serde(deny_unknown_fields)]
pub enum Validator {
    /// supported events 'pull_request.*', 'pull_request_review.*',
    #[serde(rename = "age")]
    Age(TheAge),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "approvals")]
    Approvals(TheApprovals),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "assignee")]
    Assignee(TheAssignee),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "author")]
    Author(TheAuthor),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'check_suite.*', status.*
    #[serde(rename = "baseRef")]
    BaseRef(TheBaseRef),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "changeset")]
    ChangeSet(TheChangeset),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "commit")]
    Commit(TheCommit),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "contents")]
    Contents(TheContents),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "dependent")]
    Dependent(TheDependent),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "description")]
    Description(TheDescription),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "headRef")]
    HeadRef(TheHeadRef),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "label")]
    Label(TheLabel),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "milestone")]
    Milestone(TheMilestone),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "project")]
    Project(TheProject),
    /// supported events 'pull_request.*', 'pull_request_review.*'
    #[serde(rename = "size")]
    Size(TheSize),
    /// supported events 'schedule.repository'
    #[serde(rename = "stale")]
    Stale(TheStale),
    /// supported events 'pull_request.*', 'pull_request_review.*', 'issues.*'
    #[serde(rename = "title")]
    Title(TheTitle),
    #[serde(rename = "and")]
    And(ValidatorAnd),
    #[serde(rename = "or")]
    Or(ValidatorOr),
    #[serde(rename = "not")]
    Not(ValidatorNot),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatorAnd(ValidatorStack);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatorOr(ValidatorStack);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatorNot(ValidatorStack);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ValidatorStack {
    #[serde(rename = "validate")]
    validate: Vec<Validator>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheAge {
    #[serde(rename = "created_at")]
    created_at: TimeClause,
    #[serde(rename = "updated_at")]
    updated_at: TimeClause,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheApprovals {
    #[serde(rename = "min")]
    min: Min,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Required>,
    #[serde(rename = "block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    block: Option<ApprovalsBlock>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<ApprovalsLimit>,
    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<ApprovalsExclude>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheAssignee {
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Min>,
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Max>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheAuthor {
    #[serde(flatten)]
    author: ChainedAndOrIncludeExcludeClause,
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheBaseRef {
    #[serde(rename = "must_include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<MustInclude>,
    #[serde(rename = "must_exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<MustExclude>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheChangeset {
    #[serde(rename = "no_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_empty: Option<NoEmpty>,
    #[serde(flatten)]
    changeset: ChangesetChain,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheCommit {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Message>,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheContents {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Files>,
    #[serde(flatten)]
    content: ContentsFilter,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheDependent {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<String>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
    #[serde(rename = "changed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    changed: Option<ChangedFiles>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheDescription {
    #[serde(rename = "no_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_empty: Option<NoEmpty>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<DescriptionChain>,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheHeadRef {
    #[serde(rename = "must_include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<MustInclude>,
    #[serde(rename = "must_exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<MustExclude>,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheLabel {
    #[serde(rename = "no_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_empty: Option<NoEmpty>,
    #[serde(flatten)]
    label: LabelChain,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheMilestone {
    #[serde(rename = "no_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_empty: Option<NoEmpty>,
    #[serde(flatten)]
    milestone: MilestoneChain,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheProject {
    #[serde(rename = "must_include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<MustInclude>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheSize {
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    r#match: Option<Vec<String>>,
    #[serde(rename = "ignore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore: Option<Vec<String>>,
    #[serde(rename = "lines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<LinesChain>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheStale {
    #[serde(rename = "days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<u32>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<OneOrMany<String> /*Vec<ResourceType>*/>,
    #[serde(rename = "ignore_drafts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_drafts: Option<bool>,
    #[serde(rename = "ignore_milestones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_milestones: Option<bool>,
    #[serde(rename = "ignore_projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_projects: Option<bool>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<LabelMatch>,
    #[serde(rename = "time_constraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    time_constraint: Option<TimeConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ResourceType {
    #[serde(rename = "pull_request")]
    PullRequest,
    #[serde(rename = "issues")]
    Issues,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TheTitle {
    #[serde(rename = "no_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_empty: Option<NoEmpty>,
    #[serde(flatten)]
    title: ChainedAndOrIncludeExcludeClauseBeginsEnds,
    #[serde(rename = "jira")]
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Jira>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalsRequired {
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<Vec<String>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalsBlock {
    #[serde(rename = "changes_requested")]
    changes_requested: bool,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalsLimit {
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<Vec<String>>,
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    owners: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalsExclude {
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangesetChain {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<ChangesetChain>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<ChangesetChain>>,
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
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Min>,
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Max>,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<FilesContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesContent {
    #[serde(rename = "added")]
    #[serde(skip_serializing_if = "Option::is_none")]
    added: Option<bool>,
    #[serde(rename = "modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<bool>,
    #[serde(rename = "removed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    removed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<MessageClause>,
    #[serde(rename = "skip_merge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_merge: Option<bool>,
    #[serde(rename = "oldest_only")]
    #[serde(skip_serializing_if = "Option::is_none")]
    oldest_only: Option<bool>,
    #[serde(rename = "newest_only")]
    #[serde(skip_serializing_if = "Option::is_none")]
    newest_only: Option<bool>,
    #[serde(rename = "single_commit_only")]
    #[serde(skip_serializing_if = "Option::is_none")]
    single_commit_only: Option<bool>,
    #[serde(rename = "message_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Files {
    #[serde(rename = "pr_diff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pr_diff: Option<bool>,
    #[serde(rename = "ignore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentsFilter {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangedFiles {
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptionChain {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<DescriptionChain>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<DescriptionChain>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabelChain {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<LabelChain>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<LabelChain>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MilestoneChain {
    #[serde(rename = "and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<MilestoneChain>>,
    #[serde(rename = "or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<MilestoneChain>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinesChain {
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<CountClause>,
    #[serde(rename = "additions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    additions: Option<CountClause>,
    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    deletions: Option<CountClause>,
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Max>,
    #[serde(rename = "ignore_comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_comments: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabelMatch {
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    r#match: Option<Vec<String>>,
    #[serde(rename = "ignore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeConstraint {
    #[serde(rename = "time_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(rename = "hours_between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_between: Option<Vec<String>>,
    #[serde(rename = "days_of_week")]
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<Vec<String>>,
}
