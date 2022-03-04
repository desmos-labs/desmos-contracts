use crate::{profiles::mock::MockProfilesQuerier, query::DesmosQuery, types::DesmosRoute};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{Coin, CustomQuery, OwnedDeps, SystemError, SystemResult};
use std::marker::PhantomData;

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier: MockQuerier<DesmosQuery> =
        MockQuerier::<DesmosQuery>::new(&[(contract_addr, contract_balance)])
            .with_custom_handler(|query| match query.route {
                DesmosRoute::Profiles => SystemResult::Ok(MockProfilesQuerier::query(query)),
                _ => {
                    let error = SystemError::Unknown {};
                    SystemResult::Err(error)
                }
            });
    OwnedDeps::<_, _, _, DesmosQuery> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::profiles::models_query::QueryUserApplicationLinkResponse;
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        profiles::{
            mock::MockProfilesQueries,
            models_query::{
                QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
                QueryBlocksResponse, QueryChainLinksResponse,
                QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
                QueryRelationshipsResponse, QueryUserChainLinkResponse,
            },
            querier::ProfilesQuerier,
        },
    };
    use cosmwasm_std::Addr;
    use std::ops::Deref;

    #[test]
    fn test_query_profile() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier.query_profile(Addr::unchecked("")).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_incoming_dtag_transfer_requests(Addr::unchecked(""), None)
            .unwrap();
        let expected = QueryIncomingDtagTransferRequestResponse {
            requests: vec![MockProfilesQueries::get_mock_dtag_transfer_request()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_relationships() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_relationships(Addr::unchecked(""), 0, None)
            .unwrap();
        let expected = QueryRelationshipsResponse {
            relationships: vec![MockProfilesQueries::get_mock_relationship()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_blocks() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_blocks(Addr::unchecked(""), 0, None)
            .unwrap();
        let expected = QueryBlocksResponse {
            blocks: vec![MockProfilesQueries::get_mock_user_block()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_chain_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_chain_links(Addr::unchecked(""), None)
            .unwrap();
        let expected = QueryChainLinksResponse {
            links: vec![MockProfilesQueries::get_mock_chain_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_user_chain_link() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_user_chain_link(Addr::unchecked(""), "".to_string(), "".to_string())
            .unwrap();
        let expected = QueryUserChainLinkResponse {
            link: MockProfilesQueries::get_mock_chain_link(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_app_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_links(Addr::unchecked(""), None)
            .unwrap();
        let expected = QueryApplicationLinksResponse {
            links: vec![MockProfilesQueries::get_mock_application_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_user_app_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_user_application_link(Addr::unchecked(""), "".to_string(), "".to_string())
            .unwrap();
        let expected = QueryUserApplicationLinkResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_link_by_client_id("".to_string())
            .unwrap();
        let expected = QueryApplicationLinkByClientIDResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        };

        assert_eq!(response, expected)
    }
}
