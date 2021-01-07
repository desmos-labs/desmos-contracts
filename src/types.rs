use std::time;
use std::array;
use cosmwasm_std::HumanAddr;

struct OptionalData {
    pub key: String,
    pub value: String,
}

struct Attachment {
    pub uri: String,
    pub mime_type: String,
    pub tags: array<String>
}

struct PollData {
    pub question: String,
    pub provided_answers: array<PollAnswer>,
    pub end_date: time,
    pub allows_multiple_answers: bool,
    pub allows_answer_edits: bool
}

struct PollAnswer {
    pub id: String,
    pub text: String,
}

struct Post {
    pub post_id: String,
    pub parent_id: String,
    pub message: String,
    pub created: time,
    pub last_edited: time,
    pub allows_comments: bool,
    pub subspace: String,
    pub optional_data: array<Optional_Data>,
    pub attachments: array<Attachment>,
    pub poll_data: array<PollData>,
    pub creator: HumanAddr
}
