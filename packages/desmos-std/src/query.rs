use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::subspaces::query_router::SubspacesQueryRoute;
use crate::types::DesmosRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQuery {
    pub route: DesmosRoute,
    pub query_data: DesmosQueryRouter,
}

impl CustomQuery for DesmosQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQueryRouter {
    Subspaces(SubspacesQueryRoute),
}

impl From<SubspacesQueryRoute> for DesmosQuery {
    fn from(query: SubspacesQueryRoute) -> Self {
        Self {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQueryRouter::Subspaces(query),
        }
    }
}
