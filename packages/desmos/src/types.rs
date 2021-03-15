use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// This file contains all the desmos related types used inside desmos' contracts

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OptionalData {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attachment {
    pub uri: String,
    pub mime_type: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PollAnswer {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PollData {
    pub question: String,
    pub provided_answers: Vec<PollAnswer>,
    pub end_date: String,
    pub allows_multiple_answers: bool,
    pub allows_answer_edits: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub post_id: String,
    pub parent_id: String,
    pub message: String,
    pub created: String,
    pub last_edited: String,
    pub allows_comments: bool,
    pub subspace: String,
    pub optional_data: Vec<OptionalData>,
    pub creator: String,
    pub attachments: Vec<Attachment>,
    pub poll_data: PollData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub post_id: String,
    pub _type: String,
    pub message: String,
    pub user: String,
}
