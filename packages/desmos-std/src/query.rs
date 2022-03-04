use cosmwasm_std::{CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{subspaces::query_router::SubspacesQuery, types::DesmosRoute};

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
    Subspaces(SubspacesQuery),
}

impl From<SubspacesQuery> for DesmosQuery {
    fn from(query: SubspacesQuery) -> Self {
        Self {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQueryRouter::Subspaces(query),
        }
    }
}