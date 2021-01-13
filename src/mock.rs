use crate::types::{Post, Report};
use cosmwasm_std::testing::MockQuerierCustomHandlerResult;
use crate::query::{PostsQuery, ReportsQuery, PostsQueryResponse, ReportsQueryResponse};
use cosmwasm_std::{QuerierResult, to_binary};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct PostsQuerier {
    posts: Vec<Post>,
}

impl PostsQuerier {
    pub fn new(posts: &[Post]) -> Self {
        PostsQuerier {
            posts: posts.to_vec()
        }
    }

    pub fn query(&self, request: &PostsQuery) -> QuerierResult {
        let query_result: MockQuerierCustomHandlerResult = match request {
            PostsQuery::Posts {} => {
                let res = PostsQueryResponse {
                    posts: self.posts.clone()
                };
                to_binary(&res).into()
            }
        };
        query_result
    }
}

#[derive(Clone, Default)]
pub struct ReportsQuerier {
    reports: HashMap<String, Vec<Report>>,
}

impl ReportsQuerier {
    pub fn new(reports: &[(&String, &[Report])]) -> Self {
        let mut map = HashMap::new();
        for (post_id, reports) in reports.iter() {
            map.insert(post_id.to_string(), reports.to_vec())
        }
        ReportsQuerier {
            reports: map
        }
    }

    pub fn query(&self, request: &ReportsQuery) -> QuerierResult {
        let query_result: MockQuerierCustomHandlerResult = match request {
            ReportsQuery::Reports { post_id} => {
                let reports = self
                    .reports
                    .get(post_id)
                    .unwrap();
                let res = ReportsQueryResponse {
                    reports: reports.clone()
                };
                to_binary(&res).into()
            }
        };
        query_result
    }
}



pub fn update_posts(posts: &[Post]) {
    PostsQuerier::new(posts);
}

pub fn update_reports(post_id: String, reports: &[Report]) {
    ReportsQuerier::new(&[(&post_id, &reports)]);
}
