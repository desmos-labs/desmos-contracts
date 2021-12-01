use crate::types::{DesmosRoute, Post, Reaction, Report};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DesmosQueryWrapper is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryWrapper {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery,
}

impl CustomQuery for DesmosQueryWrapper {}

/// DesmosQuery represents the available desmos network queries
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Posts {},
    Reactions { post_id: String },
    Reports { post_id: String },
}

/// PostsResponse contains a list of posts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostsResponse {
    pub posts: Vec<Post>,
}

/// ReportsResponse contains the list of reports associated with the given post_id
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReportsResponse {
    pub reports: Vec<Report>,
}

/// ReactionsResponse contains the list of reactions associated to the given post_id
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReactionsResponse {
    pub reactions: Vec<Reaction>,
}
