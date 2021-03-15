use crate::query_types::{
    DesmosQuery, DesmosQueryWrapper, DesmosRoute, PostsResponse, ReportsResponse,
};
use cosmwasm_std::{QuerierWrapper, StdResult};

pub fn query_posts(querier: &QuerierWrapper) -> StdResult<PostsResponse> {
    let request = DesmosQueryWrapper {
        route: DesmosRoute::Posts,
        query_data: DesmosQuery::Posts {},
    };

    let res: PostsResponse = querier.custom_query(&request.into())?;
    Ok(res)
}

pub fn query_post_reports(querier: &QuerierWrapper, post_id: String) -> StdResult<ReportsResponse> {
    let request = DesmosQueryWrapper {
        route: DesmosRoute::Reports,
        query_data: DesmosQuery::Reports { post_id },
    };

    let res: ReportsResponse = querier.custom_query(&request.into())?;
    Ok(res)
}
