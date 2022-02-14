use std::io::Bytes;
use crate::query_types::DesmosQueryWrapper;
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    key: [u8],
    offset: u64,
    limit: u64,
    count_total: bool,
    reverse: bool,
}
