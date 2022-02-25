use crate::{
    profiles::{
        mocks::MockProfilesQueries,
        models_app_links::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryUserApplicationLinkResponse,
        },
        models_blocks::QueryBlocksResponse,
        models_chain_links::{QueryChainLinksResponse, QueryUserChainLinkResponse},
        models_dtag_requests::QueryIncomingDtagTransferRequestResponse,
        models_profile::QueryProfileResponse,
        models_relationships::QueryRelationshipsResponse,
        query_routes::ProfilesRoutes::{
            AppLinks, ApplicationLinkByChainID, Blocks, ChainLinks, IncomingDtagTransferRequests,
            Profile, Relationships, UserAppLinks, UserChainLink,
        },
    },
    query_router::{DesmosQuery, DesmosQueryRouter},
};
use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    to_binary, Binary, Coin, ContractResult, OwnedDeps, SystemResult,
};
use std::marker::PhantomData;

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQueryRouter>, DesmosQueryRouter> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier: MockQuerier<DesmosQueryRouter> =
        MockQuerier::new(&[(contract_addr, contract_balance)]).with_custom_handler(|query| {
            SystemResult::Ok(DesmosQueryRouter::custom_query_execute(query))
        });
    OwnedDeps::<_, _, _, DesmosQueryRouter> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub trait MockQueries<T> {
    fn custom_query_execute(query: &T) -> ContractResult<Binary>;
}

impl MockQueries<DesmosQueryRouter> for DesmosQueryRouter {
    fn custom_query_execute(query: &DesmosQueryRouter) -> ContractResult<Binary> {
        let response = match query.query_data {
            DesmosQuery::Profiles(Profile { .. }) => {
                let profile = MockProfilesQueries::get_mock_profile();
                to_binary(&QueryProfileResponse { profile })
            }
            DesmosQuery::Profiles(IncomingDtagTransferRequests { .. }) => {
                let dtag_transfer_request = MockProfilesQueries::get_mock_dtag_transfer_request();
                to_binary(&QueryIncomingDtagTransferRequestResponse {
                    requests: vec![dtag_transfer_request],
                    pagination: Default::default(),
                })
            }
            DesmosQuery::Profiles(Relationships { .. }) => {
                let relationship = MockProfilesQueries::get_mock_relationship();
                to_binary(&QueryRelationshipsResponse {
                    relationships: vec![relationship],
                    pagination: Default::default(),
                })
            }
            DesmosQuery::Profiles(Blocks { .. }) => {
                let block = MockProfilesQueries::get_mock_user_block();
                to_binary(&QueryBlocksResponse {
                    blocks: vec![block],
                    pagination: Default::default(),
                })
            }
            DesmosQuery::Profiles(ChainLinks { .. }) => to_binary(&QueryChainLinksResponse {
                links: vec![MockProfilesQueries::get_mock_chain_link()],
                pagination: Default::default(),
            }),
            DesmosQuery::Profiles(UserChainLink { .. }) => to_binary(&QueryUserChainLinkResponse {
                link: MockProfilesQueries::get_mock_chain_link(),
            }),
            DesmosQuery::Profiles(AppLinks { .. }) => to_binary(&QueryApplicationLinksResponse {
                links: vec![MockProfilesQueries::get_mock_application_link()],
                pagination: Default::default(),
            }),
            DesmosQuery::Profiles(UserAppLinks { .. }) => {
                to_binary(&QueryUserApplicationLinkResponse {
                    link: MockProfilesQueries::get_mock_application_link(),
                })
            }
            DesmosQuery::Profiles(ApplicationLinkByChainID { .. }) => {
                to_binary(&QueryApplicationLinkByClientIDResponse {
                    link: MockProfilesQueries::get_mock_application_link(),
                })
            }
        };
        response.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DesmosRoute;
    use cosmwasm_std::{from_binary, Addr, QuerierWrapper, Uint64};

    #[test]
    fn test_query_profile() {
        let profile = MockProfilesQueries::get_mock_profile();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Profile {
                user: profile.account.address.clone(),
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryProfileResponse = from_binary(&bz).unwrap();
        assert_eq!(response, QueryProfileResponse { profile })
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let incoming_dtag_transfer_req = MockProfilesQueries::get_mock_dtag_transfer_request();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(IncomingDtagTransferRequests {
                receiver: incoming_dtag_transfer_req.receiver.clone(),
                pagination: None,
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryIncomingDtagTransferRequestResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryIncomingDtagTransferRequestResponse {
                requests: vec![incoming_dtag_transfer_req],
                pagination: Default::default()
            }
        )
    }

    #[test]
    fn test_query_relationships() {
        let relationship = MockProfilesQueries::get_mock_relationship();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Relationships {
                user: relationship.creator.clone(),
                subspace_id: Uint64(1),
                pagination: None,
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryRelationshipsResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryRelationshipsResponse {
                relationships: vec![relationship],
                pagination: Default::default()
            }
        )
    }

    #[test]
    fn test_query_blocks() {
        let block = MockProfilesQueries::get_mock_user_block();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Blocks {
                user: block.blocker.clone(),
                subspace_id: Uint64(1),
                pagination: None,
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryBlocksResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryBlocksResponse {
                blocks: vec![block],
                pagination: Default::default()
            }
        )
    }

    #[test]
    fn test_query_chain_links() {
        let chain_links = MockProfilesQueries::get_mock_chain_link();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(ChainLinks {
                user: chain_links.user.clone(),
                pagination: None,
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryChainLinksResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryChainLinksResponse {
                links: vec![chain_links],
                pagination: Default::default()
            }
        )
    }

    #[test]
    fn test_query_user_chain_link() {
        let user_chain_link = MockProfilesQueries::get_mock_chain_link();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(UserChainLink {
                user: user_chain_link.user.clone(),
                chain_name: user_chain_link.chain_config.name.clone(),
                target: user_chain_link.address.value.clone(),
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryUserChainLinkResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryUserChainLinkResponse {
                link: user_chain_link
            }
        )
    }

    #[test]
    fn test_query_app_links() {
        let app_link = MockProfilesQueries::get_mock_application_link();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(AppLinks {
                user: app_link.user.clone(),
                pagination: None,
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryApplicationLinksResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryApplicationLinksResponse {
                links: vec![app_link],
                pagination: Default::default()
            }
        )
    }

    #[test]
    fn test_query_user_app_links() {
        let app_link = MockProfilesQueries::get_mock_application_link();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(UserAppLinks {
                user: app_link.user.clone(),
                application: app_link.data.application.clone(),
                username: app_link.data.username.clone(),
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryUserApplicationLinkResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryUserApplicationLinkResponse { link: app_link }
        )
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let app_link = MockProfilesQueries::get_mock_application_link();
        let desmos_query_router = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(ApplicationLinkByChainID {
                client_id: app_link.oracle_request.client_id.clone(),
            }),
        };
        let bz = DesmosQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryApplicationLinkByClientIDResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryApplicationLinkByClientIDResponse { link: app_link }
        )
    }

    #[test]
    fn custom_querier() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let req = DesmosQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: DesmosQuery::Profiles(Profile {
                user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            }),
        }
        .into();
        let wrapper: QuerierWrapper<'_, DesmosQueryRouter> = QuerierWrapper::new(&deps.querier);
        let response: QueryProfileResponse = wrapper.query(&req).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        assert_eq!(response, expected);
    }
}
