use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::query_router::ProfilesQuery, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryRouter {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery,
}

impl CustomQuery for DesmosQueryRouter {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Profiles(ProfilesQuery),
}

impl From<ProfilesQuery> for DesmosQueryRouter {
    fn from(query: ProfilesQuery) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(query),
        }
    }
}
