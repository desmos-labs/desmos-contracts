use crate::{
    profiles::{
        models_app_links::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryUserApplicationLinkResponse,
        },
        models_blocks::QueryBlocksResponse,
        models_chain_links::{QueryChainLinksResponse, QueryUserChainLinkResponse},
        models_dtag_requests::QueryIncomingDtagTransferRequestResponse,
        models_profile::QueryProfileResponse,
        models_relationships::QueryRelationshipsResponse,
    },
    types::PageRequest,
};
use cosmwasm_std::{Addr, StdResult, Uint64};

pub trait ProfilesQuerier {
    fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse>;
    fn query_relationships(
        &self,
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRelationshipsResponse>;
    fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDtagTransferRequestResponse>;
    fn query_blocks(
        &self,
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse>;
    fn query_chain_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse>;
    fn query_user_chain_link(
        &self,
        user: Addr,
        chain_name: String,
        target: String,
    ) -> StdResult<QueryUserChainLinkResponse>;
    fn query_application_links(
        &self,
        user: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse>;
    fn query_user_application_link(
        &self,
        user: Addr,
        application: String,
        username: String,
    ) -> StdResult<QueryUserApplicationLinkResponse>;
    fn query_application_link_by_client_id(
        &self,
        client_id: String,
    ) -> StdResult<QueryApplicationLinkByClientIDResponse>;
}
