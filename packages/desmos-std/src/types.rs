use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint64, Binary};
use crate::query_router::DesmosQueryRouter;

pub type Deps<'a, T> = cosmwasm_std::Deps<'a, DesmosQueryRouter<T>>;
pub type DepsMut<'a, T> = cosmwasm_std::DepsMut<'a, DesmosQueryRouter<T>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosRoute {
    Subspaces,
    Profiles,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    key: Option<Binary>,
    offset: Uint64,
    limit: Uint64,
    count_total: bool,
    reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    next_key: Option<Binary>,
    total: Uint64,
}
