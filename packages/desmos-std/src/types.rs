use cosmwasm_std::{Binary, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosRoute {
    Subspaces,
    Profiles,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    key: Option<Binary>,
    offset: Uint64,
    limit: Uint64,
    count_total: bool,
    reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    next_key: Option<Binary>,
    total: Uint64,
}
