use crate::{
    relationships::models::{Relationship, UserBlock},
    types::PageResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/** UserBlocks query models **/
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryBlocksResponse {
    pub blocks: Vec<UserBlock>,
    pub pagination: PageResponse,
}

/** Relationships query models **/
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRelationshipsResponse {
    pub relationships: Vec<Relationship>,
    pub pagination: PageResponse,
}
