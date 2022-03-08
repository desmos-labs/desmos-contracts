use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{Coin, CustomQuery, OwnedDeps, SystemError, SystemResult};
use std::marker::PhantomData;

use crate::subspaces::mock::MockSubspacesQuerier;
use crate::{query::DesmosQuery, types::DesmosRoute};

pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockQuerier::<DesmosQuery>::new(&[(contract_addr, contract_balance)])
        .with_custom_handler(|query| match query.route {
            DesmosRoute::Subspaces => SystemResult::Ok(MockSubspacesQuerier::query(query)),
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
    use std::ops::Deref;
    use cosmwasm_std::Uint64;
    
    use super::*;
    use crate::subspaces::{
        mock::MockSubspacesQueries, querier::SubspacesQuerier, query_types::QuerySubspaceResponse,
    };

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
