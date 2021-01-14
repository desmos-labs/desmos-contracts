use crate::types::{Post, Report};
use cosmwasm_std::{CustomQuery, QuerierWrapper, QueryRequest, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Posts{},
    Reports{ post_id: String }
}

impl CustomQuery for DesmosQuery {}

/// PostsQueryResponse contains a list of posts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostsQueryResponse {
    pub posts: Vec<Post>,
}

pub fn query_posts(querier: &QuerierWrapper) -> StdResult<Vec<Post>> {
    let request: QueryRequest<DesmosQuery> = DesmosQuery::Posts {}.into();

    let res: PostsQueryResponse = querier.custom_query(&request)?;
    Ok(res.posts)
}

/// ReportsQueryResponse contains the list of reports associated with the given post_id
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReportsQueryResponse {
    pub reports: Vec<Report>,
}

pub fn query_post_reports(querier: &QuerierWrapper, post_id: String) -> StdResult<Vec<Report>> {
    let request: QueryRequest<DesmosQuery> = DesmosQuery::Reports { post_id }.into();

    let res: ReportsQueryResponse = querier.custom_query(&request)?;
    Ok(res.reports)
}
