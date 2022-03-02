use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Coin, CustomQuery, OwnedDeps, SystemResult,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    subspaces::{
        mocks::MockSubspacesQuerier,
        query_router::{SubspacesQueryRoute, SubspacesQueryRouter},
    },
    types::DesmosRoute,
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
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosMockRouter>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockQuerier::<DesmosMockRouter>::new(&[(contract_addr, contract_balance)])
        .with_custom_handler(|query| match &query.query_data {
            DesmosMockRoute::Subspaces(data) => {
                let c = SubspacesQueryRouter {
                    route: DesmosRoute::Subspaces,
                    query_data: data.clone(),
                };
                SystemResult::Ok(MockSubspacesQuerier::custom_query_execute(c))
            }
        });
    OwnedDeps::<_, _, _, DesmosMockRouter> {
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
        use std::ops::Deref;
        use cosmwasm_std::{from_binary, Addr, QuerierWrapper};
        use crate::{subspaces::{query_router::SubspacesQueryRoute, querier::SubspacesQuerier}, types::DesmosRoute};


        let deps = mock_dependencies_with_custom_querier(&[]);
      
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response: QueryProfileResponse = wrapper.query(&req).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        assert_eq!(response, expected);
    }
}
