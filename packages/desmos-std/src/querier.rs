use crate::{
    queries::{DesmosQueryWrapper, DesmosQuery},
    types::DesmosRoute,
};
use cosmwasm_std::{Addr, QuerierWrapper, StdResult};
use crate::profiles::models_profile::QueryProfileResponse;
use crate::profiles::models_relationships::QueryRelationshipsResponse;
use crate::types::PageRequest;

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>) -> Self {
        DesmosQuerier { querier }
    }

    pub fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = DesmosQueryWrapper{
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profile { user }
        };

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_relationships(&self, user: Addr, subspace_id: String, pagination: Option<PageRequest>)
    -> StdResult<QueryRelationshipsResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Relationships {
                user,
                subspace_id,
                pagination
            }
        };

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

}
