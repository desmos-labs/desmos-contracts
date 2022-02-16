use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::types::{PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Relationship {
    pub creator: String,
    pub recipient: String,
    pub subspace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRelationshipsResponse {
    pub relationships: Vec<Relationship>,
    pub pagination: Box<PageResponse>
}

