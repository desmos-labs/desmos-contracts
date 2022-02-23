use crate::query_router::DesmosQueryRouter;
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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    key: Vec<u8>,
    offset: u64,
    limit: u64,
    count_total: bool,
    reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    next_key: Option<String>,
    #[serde(default)]
    total: u64,
}