use cosmwasm_std::{Addr, QuerierWrapper, StdResult};
use crate::{
    queries::{DesmosQueryWrapper, DesmosQuery},
    types::{DesmosRoute, PageRequest},
    profiles::{
        models_app_links::{QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse, QueryUserApplicationLinkResponse},
        models_blocks::QueryBlocksResponse,
        models_chain_links::{QueryChainLinksResponse, QueryUserChainLinkResponse},
        models_dtag_requests::QueryIncomingDtagTransferRequestResponse,
        models_profile::QueryProfileResponse,
        models_relationships::QueryRelationshipsResponse,
    }
};

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

    pub fn query_relationships(&self, user: Addr, subspace_id: u64, pagination: Option<PageRequest>)
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

    pub fn query_incoming_dtag_transfer_request(&self, receiver: Addr, pagination: Option<PageRequest>)
        -> StdResult<QueryIncomingDtagTransferRequestResponse>{
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::IncomingDtagTransferRequests { receiver, pagination }
        };

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_blocks(&self, user: Addr, subspace_id: u64, pagination: Option<PageRequest>)
    -> StdResult<QueryBlocksResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Blocks { user, subspace_id, pagination }
        };

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_chain_links(&self, user: Addr, pagination: Option<PageRequest>)
    -> StdResult<QueryChainLinksResponse> {
        let request = DesmosQueryWrapper{
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::ChainLinks { user, pagination }
        };

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_chain_link(&self, user: Addr, chain_name: String, target: String)
        -> StdResult<QueryUserChainLinkResponse> {
        let request = DesmosQueryWrapper{
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::UserChainLink {
                user,
                chain_name,
                target
            }
        };

        let res: QueryUserChainLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_links(&self, user: Addr, pagination: Option<PageRequest>)
    -> StdResult<QueryApplicationLinksResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::AppLinks { user, pagination }
        };

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_application_link(&self, user: Addr, application: String, username: String)
    -> StdResult<QueryUserApplicationLinkResponse> {
        let request = DesmosQueryWrapper{
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::UserAppLinks {
                user,
                application,
                username
            }
        };

        let res: QueryUserApplicationLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_link_by_client_id(&self, client_id: String)
    -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = DesmosQueryWrapper {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::ApplicationLinkByChainID { client_id }
        };

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
