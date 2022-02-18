use crate::query::DesmosQueryRouter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Deps<'a> = cosmwasm_std::Deps<'a, DesmosQueryRouter>;
pub type DepsMut<'a> = cosmwasm_std::DepsMut<'a, DesmosQueryRouter>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosRoute {
    Subspaces,
    Profiles,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PageRequest {
    key: Vec<u8>,
    offset: u64,
    limit: u64,
    count_total: bool,
    reverse: bool,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PageResponse {
    next_key: Vec<u8>,
    total: String,
}
