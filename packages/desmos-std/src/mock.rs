use crate::{
    profiles::mock::MockProfilesQuerier, query::DesmosQuery, subspaces::mock::MockSubspacesQuerier,
};
use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Coin, CustomQuery, OwnedDeps, SystemResult,
};
use std::marker::PhantomData;

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockQuerier::<DesmosQuery>::new(&[(contract_addr, contract_balance)])
        .with_custom_handler(|query| match query {
            DesmosQuery::Profiles(query) => {
                SystemResult::Ok(MockProfilesQuerier::query(query))
            }
            DesmosQuery::Subspaces(query) => {
                SystemResult::Ok(MockSubspacesQuerier::query(query))
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
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        profiles::{
            mock::MockProfilesQueries, models_query::QueryProfileResponse, querier::ProfilesQuerier,
        },
        subspaces::{
            mock::MockSubspacesQueries, querier::SubspacesQuerier,
            query_types::QuerySubspaceResponse,
        },
    };
    use cosmwasm_std::Addr;
    use cosmwasm_std::Uint64;
    use std::ops::Deref;

    #[test]
    fn test_profiles_querier_mock() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(deps.querier.deref());
        let response = querier.query_profile(Addr::unchecked("")).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_subspaces_querier() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(Uint64::new(1)).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(response, expected);
    }
}
