use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::query_router::ProfilesQuery, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQuery {
    pub route: DesmosRoute,
    pub query_data: DesmosQueryRoute,
}

impl CustomQuery for DesmosQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQueryRoute {
    Profiles(ProfilesQuery),
}

impl From<ProfilesQuery> for DesmosQuery {
    fn from(query: ProfilesQuery) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(query),
        }
    }
}
