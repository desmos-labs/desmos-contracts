use crate::types::PageResponse;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserBlock {
    pub blocker: Addr,
    pub blocked: Addr,
    pub reason: String,
    pub subspace_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryBlocksResponse {
    pub blocks: Vec<UserBlock>,
    pub pagination: PageResponse,
}
