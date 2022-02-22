use cosmwasm_std::{Addr, QuerierWrapper, StdResult};
use crate::{
    query_router::{DesmosQueryRouter, DesmosQuery},
    types::{DesmosRoute, PageRequest},
    profiles::{
        models_app_links::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryUserApplicationLinkResponse
        },
        models_blocks::QueryBlocksResponse,
        models_chain_links::{QueryChainLinksResponse, QueryUserChainLinkResponse},
        models_dtag_requests::QueryIncomingDtagTransferRequestResponse,
        models_profile::QueryProfileResponse,
        models_relationships::QueryRelationshipsResponse,
        querier::ProfilesQuerier,
        query_routes::ProfilesRoutes::{
            ApplicationLinkByChainID, AppLinks, Blocks, ChainLinks,
            IncomingDtagTransferRequests, Profile, Relationships, UserAppLinks, UserChainLink
        },
    }
};

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryRouter>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryRouter>) -> Self {
        DesmosQuerier { querier }
    }
}

impl <'a> ProfilesQuerier for DesmosQuerier<'a> {
    fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Profile { user })
        };

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_relationships(&self, user: Addr, subspace_id: u64, pagination: Option<PageRequest>)
                               -> StdResult<QueryRelationshipsResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Relationships {
                user,
                subspace_id,
                pagination
            })
        };

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_incoming_dtag_transfer_requests(&self, receiver: Addr, pagination: Option<PageRequest>)
                                                 -> StdResult<QueryIncomingDtagTransferRequestResponse>{
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(IncomingDtagTransferRequests { receiver, pagination })
        };

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_blocks(&self, user: Addr, subspace_id: u64, pagination: Option<PageRequest>)
                        -> StdResult<QueryBlocksResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Blocks { user, subspace_id, pagination })
        };

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_chain_links(&self, user: Addr, pagination: Option<PageRequest>)
                             -> StdResult<QueryChainLinksResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(ChainLinks { user, pagination })
        };

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_chain_link(&self, user: Addr, chain_name: String, target: String)
                                 -> StdResult<QueryUserChainLinkResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(UserChainLink {
                user,
                chain_name,
                target
            })
        };

        let res: QueryUserChainLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_application_links(&self, user: Addr, pagination: Option<PageRequest>)
                                   -> StdResult<QueryApplicationLinksResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(AppLinks { user, pagination })
        };

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_application_link(&self, user: Addr, application: String, username: String)
                                       -> StdResult<QueryUserApplicationLinkResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(UserAppLinks {
                user,
                application,
                username
            })
        };

        let res: QueryUserApplicationLinkResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_application_link_by_client_id(&self, client_id: String)
                                               -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(ApplicationLinkByChainID { client_id })
        };

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
