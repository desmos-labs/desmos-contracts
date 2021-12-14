use crate::{
    query_types::{
        DesmosQuery, DesmosQueryWrapper, PostsResponse, ReactionsResponse, ReportsResponse,
    },
    types::DesmosRoute,
};
use cosmwasm_std::{QuerierWrapper, StdResult};

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>) -> Self {
        DesmosQuerier { querier }
    }

    pub fn query_posts(&self) -> StdResult<PostsResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Posts {},
        };

        let res: PostsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_post_reports(&self, post_id: String) -> StdResult<ReportsResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Reports { post_id },
        };

        let res: ReportsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_post_reactions(&self, post_id: String) -> StdResult<ReactionsResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Reactions { post_id },
        };

        let res: ReactionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
