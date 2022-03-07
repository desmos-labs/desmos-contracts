use crate::{profiles::mock::MockProfilesQuerier, query::DesmosQuery, types::DesmosRoute};
use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Coin, CustomQuery, OwnedDeps, SystemError, SystemResult,
};
use std::marker::PhantomData;

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier: MockQuerier<DesmosQuery> =
        MockQuerier::<DesmosQuery>::new(&[(contract_addr, contract_balance)]).with_custom_handler(
            |query| match query.route {
                DesmosRoute::Profiles => SystemResult::Ok(MockProfilesQuerier::query(query)),
                _ => {
                    let error = SystemError::Unknown {};
                    SystemResult::Err(error)
                }
            },
        );
    OwnedDeps::<_, _, _, DesmosQuery> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        profiles::{
            mock::{MockProfilesQuerier, MockProfilesQueries},
            models_query::{
                QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
                QueryBlocksResponse, QueryChainLinksResponse,
                QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
                QueryRelationshipsResponse, QueryUserApplicationLinkResponse,
                QueryUserChainLinkResponse,
            },
            query_router::ProfilesQuery,
        },
        query::DesmosQuery,
    };
    use cosmwasm_std::{to_binary, Addr};

    #[test]
    fn test_query_profile() {
        let query = ProfilesQuery::Profile {
            user: Addr::unchecked(""),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let query = ProfilesQuery::IncomingDtagTransferRequests {
            receiver: Addr::unchecked(""),
            pagination: Default::default(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryIncomingDtagTransferRequestResponse {
            requests: vec![MockProfilesQueries::get_mock_dtag_transfer_request()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_relationships() {
        let query = ProfilesQuery::Relationships {
            user: Addr::unchecked(""),
            subspace_id: 0,
            pagination: Default::default(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryRelationshipsResponse {
            relationships: vec![MockProfilesQueries::get_mock_relationship()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_blocks() {
        let query = ProfilesQuery::Blocks {
            user: Addr::unchecked(""),
            subspace_id: 0,
            pagination: Default::default(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryBlocksResponse {
            blocks: vec![MockProfilesQueries::get_mock_user_block()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_chain_links() {
        let query = ProfilesQuery::ChainLinks {
            user: Addr::unchecked(""),
            pagination: Default::default(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryChainLinksResponse {
            links: vec![MockProfilesQueries::get_mock_chain_link()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_user_chain_link() {
        let query = ProfilesQuery::UserChainLink {
            user: Addr::unchecked(""),
            chain_name: "cosmos".to_string(),
            target: "".to_string(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryUserChainLinkResponse {
            link: MockProfilesQueries::get_mock_chain_link(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_app_links() {
        let query = ProfilesQuery::AppLinks {
            user: Addr::unchecked(""),
            pagination: Default::default(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryApplicationLinksResponse {
            links: vec![MockProfilesQueries::get_mock_application_link()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_user_app_links() {
        let query = ProfilesQuery::UserAppLinks {
            user: Addr::unchecked(""),
            application: "".to_string(),
            username: "".to_string(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryUserApplicationLinkResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let query = ProfilesQuery::ApplicationLinkByChainID {
            client_id: "".to_string(),
        };
        let response = MockProfilesQuerier::query(&DesmosQuery::from(query));
        let expected = to_binary(&QueryApplicationLinkByClientIDResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }
}
