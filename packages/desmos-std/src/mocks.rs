use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Coin, CustomQuery, OwnedDeps, SystemResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    subspaces::{
        mocks::MockSubspacesQuerier,
        query_router::{SubspacesQueryRoute, SubspacesQueryRouter},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMockRoute {
    Subspaces(SubspacesQueryRoute),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMockRouter {
    query_data: DesmosMockRoute,
}

impl CustomQuery for DesmosMockRouter {}

impl DesmosMockRouter {
    pub fn new(query_data: DesmosMockRoute) -> DesmosMockRouter {
        Self { query_data }
    }
}

pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<SubspacesQueryRouter>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockQuerier::<SubspacesQueryRouter>::new(&[(contract_addr, contract_balance)])
        .with_custom_handler(|query|  {
            SystemResult::Ok(MockSubspacesQuerier::custom_query_execute(query.clone()))
        });
    OwnedDeps::<_, _, _, SubspacesQueryRouter> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn custom_querier() {
        use super::*;
        use crate::subspaces::{
            mocks::MockSubspacesQueries, querier::SubspacesQuerier,
            query_types::QuerySubspaceResponse,
        };
        use cosmwasm_std::Uint64;
        use std::ops::Deref;

        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(Uint64::new(1)).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        println!("response {:?}", response);
        assert_eq!(response, expected);
    }
}
