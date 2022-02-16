use crate::{
    queries::{DesmosQueryWrapper},
    types::DesmosRoute,
};
use cosmwasm_std::{QuerierWrapper, StdResult};
use crate::profiles::models_profile::QueryProfileResponse;
use crate::queries::DesmosQuery::QueryProfileRequest;

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>) -> Self {
        DesmosQuerier { querier }
    }

    pub fn query_profile(&self, user: String) -> StdResult<QueryProfileResponse> {
        let request = DesmosQueryWrapper{
            route: DesmosRoute::Profiles,
            query_data: QueryProfileRequest { user }
        };

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
