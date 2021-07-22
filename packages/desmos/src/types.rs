use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// This file contains all the desmos related types used inside desmos' contracts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attachment {
    pub uri: String,
    pub mime_type: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProvidedAnswer {
    pub answer_id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Poll {
    pub question: String,
    pub provided_answers: Vec<ProvidedAnswer>,
    pub end_date: String,
    pub allows_multiple_answers: bool,
    pub allows_answer_edits: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub post_id: String,
    pub parent_id: Option<String>,
    pub message: String,
    pub created: String,
    pub last_edited: String,
    pub comments_state: String,
    pub subspace: String,
    pub additional_attributes: Option<Vec<Attribute>>,
    pub creator: String,
    pub attachments: Option<Vec<Attachment>>,
    pub poll: Option<Poll>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub post_id: String,
    pub kind: String,
    pub message: String,
    pub user: String,
}
