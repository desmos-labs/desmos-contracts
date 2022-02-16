use crate::queries::DesmosQueryWrapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Deps<'a> = cosmwasm_std::Deps<'a, DesmosQueryWrapper>;
pub type DepsMut<'a> = cosmwasm_std::DepsMut<'a, DesmosQueryWrapper>;

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
    total: u64,
}
