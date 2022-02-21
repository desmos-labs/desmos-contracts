use crate::{
    query::{DesmosQueryRouter, DesmosQuery, SubspacesQuery},
    types::{DesmosRoute, PageRequest},
    subspaces::query::QuerySubspacesResponse
};
use cosmwasm_std::{QuerierWrapper, StdResult};

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryRouter>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryRouter>) -> Self {
        DesmosQuerier { querier }
    }
}

pub trait SubspacesQuerier {
    fn query_subspaces(&self, pagination: Option<PageRequest>) ->  StdResult<QuerySubspacesResponse>;
}

impl <'a> SubspacesQuerier for DesmosQuerier<'a> {
    fn query_subspaces(&self, pagination: Option<PageRequest>) ->  StdResult<QuerySubspacesResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(
                SubspacesQuery::Subspaces{
                pagination: pagination
            })
        };
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}