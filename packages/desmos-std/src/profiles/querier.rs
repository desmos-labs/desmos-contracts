use crate::{
    profiles::{
        models_query::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryBlocksResponse, QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
            QueryProfileResponse, QueryRelationshipsResponse, QueryUserApplicationLinkResponse,
            QueryUserChainLinkResponse,
        },
        query_router::{ProfilesQueryRoute, ProfilesQueryRouter},
    },
    types::{DesmosRoute, PageRequest},
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult, Uint64};

pub struct ProfilesQuerier<'a> {
    querier: QuerierWrapper<'a, ProfilesQueryRouter>,
}

impl<'a> ProfilesQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, ProfilesQueryRouter>::new(querier),
        }
    }

    fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Profile { user },
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
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Relationships {
                user,
                subspace_id,
                pagination,
            },
        };

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDtagTransferRequestResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::IncomingDtagTransferRequests {
                receiver,
                pagination,
            },
        };

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_blocks(
        &self,
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Blocks {
                user,
                subspace_id,
                pagination,
            },
        };

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_chain_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::ChainLinks { user, pagination },
        };

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_chain_link(
        &self,
        user: Addr,
        chain_name: String,
        target: String,
    ) -> StdResult<QueryUserChainLinkResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::UserChainLink {
                user,
                chain_name,
                target,
            },
        };

        let res: QueryUserChainLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_application_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::AppLinks { user, pagination },
        };

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_application_link(
        &self,
        user: Addr,
        application: String,
        username: String,
    ) -> StdResult<QueryUserApplicationLinkResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::UserAppLinks {
                user,
                application,
                username,
            },
        };

        let res: QueryUserApplicationLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_application_link_by_client_id(
        &self,
        client_id: String,
    ) -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::ApplicationLinkByChainID { client_id },
        };

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
