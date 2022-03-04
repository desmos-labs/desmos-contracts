use crate::{
    profiles::{
        models_query::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryBlocksResponse, QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
            QueryProfileResponse, QueryRelationshipsResponse, QueryUserApplicationLinkResponse,
            QueryUserChainLinkResponse,
        },
        query_router::ProfilesQuery,
    },
    query::{DesmosQuery, DesmosQueryRoute},
    types::{DesmosRoute, PageRequest},
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};

pub struct ProfilesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> ProfilesQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }

    pub fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::Profile { user }),
        };

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_relationships(
        &self,
        user: Addr,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRelationshipsResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::Relationships {
                user,
                subspace_id,
                pagination,
            }),
        };

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDtagTransferRequestResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::IncomingDtagTransferRequests {
                receiver,
                pagination,
            }),
        };

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_blocks(
        &self,
        user: Addr,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::Blocks {
                user,
                subspace_id,
                pagination,
            }),
        };

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_chain_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::ChainLinks { user, pagination }),
        };

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_chain_link(
        &self,
        user: Addr,
        chain_name: String,
        target: String,
    ) -> StdResult<QueryUserChainLinkResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::UserChainLink {
                user,
                chain_name,
                target,
            }),
        };

        let res: QueryUserChainLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::AppLinks { user, pagination }),
        };

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_application_link(
        &self,
        user: Addr,
        application: String,
        username: String,
    ) -> StdResult<QueryUserApplicationLinkResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::UserAppLinks {
                user,
                application,
                username,
            }),
        };

        let res: QueryUserApplicationLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_link_by_client_id(
        &self,
        client_id: String,
    ) -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(ProfilesQuery::ApplicationLinkByChainID {
                client_id,
            }),
        };

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
