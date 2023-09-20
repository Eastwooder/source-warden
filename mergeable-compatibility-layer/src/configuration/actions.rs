use serde::{Deserialize, Serialize};

use super::basics::OneOrMany;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssignAction {
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<OneOrMany<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckAction {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Payload>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Payload {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloseAction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentAction {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<CommentPayload>,
    #[serde(rename = "leave_old_comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    leave_old_comment: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentPayload {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MergeAction {
    #[serde(rename = "merge_method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<String>,
    #[serde(rename = "commit_title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_title: Option<String>,
    #[serde(rename = "commit_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabelsAction {
    #[serde(rename = "add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    add: Option<OneOrMany<String>>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<OneOrMany<String>>,
    #[serde(rename = "replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<OneOrMany<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestReviewAction {
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<OneOrMany<String>>,
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<OneOrMany<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "do")]
#[serde(deny_unknown_fields)]
pub enum Action {
    /// Supported Events 'pull_request.*', 'issues.*'
    #[serde(rename = "assign")]
    Assign(AssignAction),
    /// Supported Events 'pull_request.assigned', 'pull_request.auto_merge_disabled', 'pull_request.auto_merge_enabled', 'pull_request.converted_to_draft', 'pull_request.demilestoned', 'pull_request.dequeued', 'pull_request.edited', 'pull_request.enqueued', 'pull_request.labeled', 'pull_request.locked', 'pull_request.milestoned', 'pull_request.opened', 'pull_request.push_synchronize', 'pull_request.ready_for_review', 'pull_request.reopened', 'pull_request.review_request_removed', 'pull_request.review_requested', 'pull_request.synchronize', 'pull_request.unassigned', 'pull_request.unlabeled', 'pull_request.unlocked', 'pull_request_review.dismissed', 'pull_request_review.edited', 'pull_request_review.submitted'
    #[serde(rename = "checks")]
    Checks(CheckAction),
    /// Supported Events 'schedule.repository', 'pull_request.*', 'issues.*'
    #[serde(rename = "close")]
    Close(CloseAction),
    /// Supported Events 'schedule.repository', 'pull_request.*', 'issues.*'
    #[serde(rename = "comment")]
    Comment(CommentAction),
    /// Supported Events 'pull_request.*', 'pull_request_review.*', 'status.*', 'check_suite.*'
    #[serde(rename = "merge")]
    Merge(MergeAction),
    /// Supported Events 'schedule.repository', 'pull_request.*', 'issues.*'
    #[serde(rename = "labels")]
    Labels(LabelsAction),
    /// Supported Events 'pull_request.*'
    #[serde(rename = "request_review")]
    RequestReview(RequestReviewAction),
}
