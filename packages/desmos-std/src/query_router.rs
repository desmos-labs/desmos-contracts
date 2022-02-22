use crate::{
    query::{DesmosQueryRouter, DesmosQuery},
    types::{DesmosRoute, PageRequest},
    subspaces::{
        query::{QuerySubspacesResponse, QuerySubspaceResponse},
        routes::SubspacesRoutes
    }
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
    fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse>;
}

impl <'a> SubspacesQuerier for DesmosQuerier<'a> {
    fn query_subspaces(&self, pagination: Option<PageRequest>) ->  StdResult<QuerySubspacesResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(
                SubspacesRoutes::Subspaces{
                pagination: pagination
            })
        };
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(
                SubspacesRoutes::Subspace{
                subspace_id : subspace_id
            })
        };
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}