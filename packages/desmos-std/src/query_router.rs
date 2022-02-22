use crate::{profiles::query_routes::ProfilesRoutes, types::DesmosRoute};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DesmosQueryRouter is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryRouter {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery,
}

impl CustomQuery for DesmosQueryRouter {}

/// DesmosQuery represents the available desmos network queries
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Profiles(ProfilesRoutes),
}
