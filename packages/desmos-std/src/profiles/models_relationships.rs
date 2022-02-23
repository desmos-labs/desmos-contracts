use crate::types::PageResponse;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Relationship {
    pub creator: Addr,
    pub recipient: Addr,
    pub subspace_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRelationshipsResponse {
    pub relationships: Vec<Relationship>,
    pub pagination: PageResponse,
}
