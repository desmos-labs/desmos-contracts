use crate::types::{Post, Reaction, Report};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosRoute {
    Posts,
    Subspaces,
    Profiles
}

/// DesmosQueryWrapper is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryWrapper<'a> {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery<'a>,
}

impl CustomQuery for DesmosQueryWrapper<'_> {}

/// DesmosQuery represents the available desmos network queries
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery<'a> {
    Posts {},
    Reactions { post_id: & 'a str },
    Reports { post_id: & 'a str },
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
