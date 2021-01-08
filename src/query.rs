use cosmwasm_std::{CustomQuery, StdResult, QuerierWrapper, Querier};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::types::{Post, Report};
use crate::query::ReportsQuery::Reports;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostsQuery {
    /// Returns a list of all the posts
    Posts{}
}

/// PostsQueryResponse contains a list of posts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostsQueryResponse {
    pub posts: Vec<Post>
}

impl CustomQuery for PostsQuery {}

pub fn query_posts(querier: &QuerierWrapper) -> StdResult<Vec<Post>> {
    let request = PostsQuery::Posts {}
        .into();

    let res: PostsQueryResponse = querier.query(&request)?;
    Ok(res.posts)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportsQuery {
    /// Return all the reports associated with the post_id given
    Reports { post_id: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReportsQueryResponse {
    pub reports: Vec<Report>
}

impl CustomQuery for ReportsQuery {}

pub fn query_reports(querier: &QuerierWrapper, post_id: String) -> StdResult<Vec<Report>> {
    let request = ReportsQuery::Reports { post_id }
        .into();

    let res: ReportsQueryResponse = querier.query(&request)?;
    Ok(res.reports)
}

