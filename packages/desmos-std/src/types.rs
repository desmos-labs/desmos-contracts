use cosmwasm_std::{Binary, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    pub key: Option<Binary>,
    pub offset: Option<Uint64>,
    pub limit: Uint64,
    pub count_total: bool,
    pub reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    pub next_key: Option<Binary>,
    pub total: Uint64,
}
