use crate::types::{Post, Report};
use cosmwasm_std::{CustomQuery, QuerierWrapper, QueryRequest, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostsQuery {
    /// Returns a list of all the posts
    Posts {},
}

/// trait that need to be implemented to avoid conflicts with cosmwasm_std custom queries
impl CustomQuery for PostsQuery {}

/// PostsQueryResponse contains a list of posts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostsQueryResponse {
    pub posts: Vec<Post>,
}

pub fn query_posts(querier: &QuerierWrapper) -> StdResult<Vec<Post>> {
    let request: QueryRequest<PostsQuery> = PostsQuery::Posts {}.into();

    let res: PostsQueryResponse = querier.custom_query(&request)?;
    Ok(res.posts)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportsQuery {
    /// Return all the reports associated with the post_id given
    Reports { post_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReportsQueryResponse {
    pub reports: Vec<Report>,
}

impl CustomQuery for ReportsQuery {}

pub fn query_post_reports(querier: &QuerierWrapper, post_id: String) -> StdResult<Vec<Report>> {
    let request: QueryRequest<ReportsQuery> = ReportsQuery::Reports { post_id }.into();

    let res: ReportsQueryResponse = querier.custom_query(&request)?;
    Ok(res.reports)
}
