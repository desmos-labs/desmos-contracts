use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserBlock {
    pub blocker: Addr,
    pub blocked: Addr,
    pub reason: String,
    pub subspace_id: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Relationship {
    pub creator: Addr,
    pub counterparty: Addr,
    pub subspace_id: Uint64,
}
