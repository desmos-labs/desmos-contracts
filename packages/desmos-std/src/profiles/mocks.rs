use crate::profiles::models_query::{
    QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse, QueryBlocksResponse,
    QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
    QueryRelationshipsResponse, QueryUserApplicationLinkResponse, QueryUserChainLinkResponse,
};
use crate::profiles::query_router::{ProfilesQueryRoute, ProfilesQueryRouter};
use crate::profiles::{
    models_app_links::{AppLinkResult, ApplicationLink, CallData, Data, OracleRequest},
    models_blocks::UserBlock,
    models_chain_links::{ChainConfig, ChainLink, ChainLinkAddr, Proof, Signature},
    models_common::PubKey,
    models_dtag_requests::DtagTransferRequest,
    models_profile::{Account, Pictures, Profile},
    models_relationships::Relationship,
};
use cosmwasm_std::{to_binary, Addr, Binary, Coin, ContractResult, OwnedDeps, SystemResult};
use std::marker::PhantomData;

/**
This file contains some useful mocks of the Desmos x/profiles modules types ready made to be used
in any test
**/

#[cfg(not(target_arch = "wasm32"))]
mod mocks {
    use std::marker::PhantomData;
    use cosmwasm_std::{Addr, Binary, Coin, ContractResult, OwnedDeps, SystemResult, to_binary};
    use cosmwasm_std::testing::{MOCK_CONTRACT_ADDR, MockApi, MockQuerier, MockStorage};
    use crate::profiles::models_app_links::{ApplicationLink, AppLinkResult, CallData, Data, OracleRequest};
    use crate::profiles::models_blocks::UserBlock;
    use crate::profiles::models_chain_links::{ChainConfig, ChainLink, ChainLinkAddr, Proof, Signature};
    use crate::profiles::models_common::PubKey;
    use crate::profiles::models_dtag_requests::DtagTransferRequest;
    use crate::profiles::models_profile::{Account, Pictures, Profile};
    use crate::profiles::models_query::{QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse, QueryBlocksResponse, QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse, QueryProfileResponse, QueryRelationshipsResponse, QueryUserApplicationLinkResponse, QueryUserChainLinkResponse};
    use crate::profiles::models_relationships::Relationship;
    use crate::profiles::query_router::{ProfilesQueryRoute, ProfilesQueryRouter};

    pub struct MockProfilesQueries {}

    impl MockProfilesQueries {
        pub fn get_mock_profile() -> Profile {
            Profile {
                account: Account {
                    proto_type: "/cosmos.auth.v1beta1.BaseAccount".to_string(),
                    address: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                    pub_key: PubKey {
                        proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                        key: "ArlRm0a5fFTHFfKha1LpDd+g3kZlyRBBF4R8PSM8Zo4Y".to_string(),
                    },
                    account_number: "0".to_string(),
                    sequence: "15".to_string(),
                },
                dtag: "goldrake".to_string(),
                nickname: "Goldrake".to_string(),
                bio: "This is Goldrake".to_string(),
                pictures: Pictures {
                    profile: "".to_string(),
                    cover: "".to_string(),
                },
                creation_date: "2022-02-21T13:18:27.257641Z".to_string(),
            }
        }

        pub fn get_mock_dtag_transfer_request() -> DtagTransferRequest {
            DtagTransferRequest {
                dtag_to_trade: "goldrake".to_string(),
                sender: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                receiver: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            }
        }

        pub fn get_mock_relationship() -> Relationship {
            Relationship {
                creator: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                recipient: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
                subspace_id: "1".to_string(),
            }
        }

        pub fn get_mock_user_block() -> UserBlock {
            UserBlock {
                blocker: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                blocked: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
                reason: "test".to_string(),
                subspace_id: "1".to_string(),
            }
        }

        pub fn get_mock_chain_link() -> ChainLink {
            ChainLink{
                user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                address: ChainLinkAddr {
                    proto_type: "/desmos.profiles.v1beta1.Bech32Address".to_string(),
                    value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
                    prefix: "cosmos".to_string()
                },
                proof: Proof {
                    pub_key: PubKey {
                        proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                        key: "AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM".to_string()
                    },
                    signature: Signature {
                        proto_type: "/desmos.profiles.v1beta1.SingleSignatureData".to_string(),
                        signature: "C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg".to_string()
                    },
                    plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string()
                },
                chain_config: ChainConfig { name: "cosmos".to_string() },
                creation_time: "2022-02-21T13:18:57.800827Z".to_string()
            }
        }

        pub fn get_mock_application_link() -> ApplicationLink {
            ApplicationLink {
                user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                data: Data {
                    application: "twitter".to_string(),
                    username: "goldrake".to_string(),
                },
                state: "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS".to_string(),
                oracle_request: OracleRequest {
                    id: "537807".to_string(),
                    oracle_script_id: "32".to_string(),
                    call_data: CallData {
                        application: "twitter".to_string(),
                        call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".to_string(),
                    },
                    client_id: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake"
                        .to_string(),
                },
                result: AppLinkResult::Success {
                    value: "4c756361675f5f2345423337".to_string(),
                    signature: "9690d734171298eb4cc9636c36d8507535264c1fdb136c9095a6a50c41ccffa"
                        .to_string(),
                },
                creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
            }
        }
    }

    /// Replacement for cosmwasm_std::testing::mock_dependencies
    /// this use our CustomQuerier to use desmos querier
    pub fn mock_dependencies_with_custom_querier(
        contract_balance: &[Coin],
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier<ProfilesQueryRouter>, ProfilesQueryRouter> {
        let contract_addr = MOCK_CONTRACT_ADDR;
        let custom_querier: MockQuerier<ProfilesQueryRouter> =
            MockQuerier::new(&[(contract_addr, contract_balance)]).with_custom_handler(|query| {
                SystemResult::Ok(ProfilesQueryRouter::custom_query_execute(query))
            });
        OwnedDeps::<_, _, _, ProfilesQueryRouter> {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: custom_querier,
            custom_query_type: PhantomData,
        }
    }

    pub trait MockQueries<T> {
        fn custom_query_execute(query: &T) -> ContractResult<Binary>;
    }

    impl MockQueries<ProfilesQueryRouter> for ProfilesQueryRouter {
        fn custom_query_execute(query: &ProfilesQueryRouter) -> ContractResult<Binary> {
            let response = match query.query_data {
                ProfilesQueryRoute::Profile { .. } => {
                    let profile = MockProfilesQueries::get_mock_profile();
                    to_binary(&QueryProfileResponse { profile })
                }
                ProfilesQueryRoute::IncomingDtagTransferRequests { .. } => {
                    let dtag_transfer_request = MockProfilesQueries::get_mock_dtag_transfer_request();
                    to_binary(&QueryIncomingDtagTransferRequestResponse {
                        requests: vec![dtag_transfer_request],
                        pagination: Default::default(),
                    })
                }
                ProfilesQueryRoute::Relationships { .. } => {
                    let relationship = MockProfilesQueries::get_mock_relationship();
                    to_binary(&QueryRelationshipsResponse {
                        relationships: vec![relationship],
                        pagination: Default::default(),
                    })
                }
                ProfilesQueryRoute::Blocks { .. } => {
                    let block = MockProfilesQueries::get_mock_user_block();
                    to_binary(&QueryBlocksResponse {
                        blocks: vec![block],
                        pagination: Default::default(),
                    })
                }
                ProfilesQueryRoute::ChainLinks { .. } => to_binary(&QueryChainLinksResponse {
                    links: vec![MockProfilesQueries::get_mock_chain_link()],
                    pagination: Default::default(),
                }),
                ProfilesQueryRoute::UserChainLink { .. } => to_binary(&QueryUserChainLinkResponse {
                    link: MockProfilesQueries::get_mock_chain_link(),
                }),
                ProfilesQueryRoute::AppLinks { .. } => to_binary(&QueryApplicationLinksResponse {
                    links: vec![MockProfilesQueries::get_mock_application_link()],
                    pagination: Default::default(),
                }),
                ProfilesQueryRoute::UserAppLinks { .. } => {
                    to_binary(&QueryUserApplicationLinkResponse {
                        link: MockProfilesQueries::get_mock_application_link(),
                    })
                }
                ProfilesQueryRoute::ApplicationLinkByChainID { .. } => {
                    to_binary(&QueryApplicationLinkByClientIDResponse {
                        link: MockProfilesQueries::get_mock_application_link(),
                    })
                }
            };
            response.into()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::models_query::{
        QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
    };
    use crate::{profiles::query_router::ProfilesQueryRoute, types::DesmosRoute};
    use cosmwasm_std::{from_binary, Addr, QuerierWrapper, Uint64};
    use crate::profiles::mocks::mocks::{mock_dependencies_with_custom_querier, MockProfilesQueries, MockQueries};

    #[test]
    fn test_query_profile() {
        let profile = MockProfilesQueries::get_mock_profile();
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Profile {
                user: profile.account.address.clone(),
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryProfileResponse = from_binary(&bz).unwrap();
        assert_eq!(response, QueryProfileResponse { profile })
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let incoming_dtag_transfer_req = MockProfilesQueries::get_mock_dtag_transfer_request();
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::IncomingDtagTransferRequests {
                receiver: incoming_dtag_transfer_req.receiver.clone(),
                pagination: None,
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Relationships {
                user: relationship.creator.clone(),
                subspace_id: 1,
                pagination: None,
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Blocks {
                user: block.blocker.clone(),
                subspace_id: Uint64::new(1),
                pagination: None,
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::ChainLinks {
                user: chain_links.user.clone(),
                pagination: None,
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::UserChainLink {
                user: user_chain_link.user.clone(),
                chain_name: user_chain_link.chain_config.name.clone(),
                target: user_chain_link.address.value.clone(),
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::AppLinks {
                user: app_link.user.clone(),
                pagination: None,
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
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
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::UserAppLinks {
                user: app_link.user.clone(),
                application: app_link.data.application.clone(),
                username: app_link.data.username.clone(),
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryUserApplicationLinkResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryUserApplicationLinkResponse { link: app_link }
        )
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let app_link = MockProfilesQueries::get_mock_application_link();
        let desmos_query_router = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::ApplicationLinkByChainID {
                client_id: app_link.oracle_request.client_id.clone(),
            },
        };
        let bz = ProfilesQueryRouter::custom_query_execute(&desmos_query_router).unwrap();
        let response: QueryApplicationLinkByClientIDResponse = from_binary(&bz).unwrap();
        assert_eq!(
            response,
            QueryApplicationLinkByClientIDResponse { link: app_link }
        )
    }

    #[test]
    fn custom_querier() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let req = ProfilesQueryRouter {
            route: DesmosRoute::Profiles,
            query_data: ProfilesQueryRoute::Profile {
                user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            },
        }
        .into();
        let wrapper: QuerierWrapper<'_, ProfilesQueryRouter> = QuerierWrapper::new(&deps.querier);
        let response: QueryProfileResponse = wrapper.query(&req).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        assert_eq!(response, expected);
    }
}
