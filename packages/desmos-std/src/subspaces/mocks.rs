use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    {to_binary, Addr, Binary, Coin, ContractResult, CustomQuery, OwnedDeps, SystemResult, Uint64},
};
use std::marker::PhantomData;

use crate::subspaces::{
    models::{GroupPermission, PermissionDetail, Subspace, UserGroup},
    query_router::{SubspacesQueryRoute, SubspacesQueryRouter},
    query_types::{
        QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
        QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
    },
};

/**
This file contains some useful mocks of the Desmos x/subspaces modules types ready made to be used
in any test
**/

pub struct MockSubspacesQueries {}

impl MockSubspacesQueries {
    pub fn get_mock_subspace() -> Subspace {
        Subspace {
            id: Uint64::new(1),
            name: "Test subspace".to_string(),
            description: "Test subspace".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
        }
    }

    pub fn get_mock_user_group() -> UserGroup {
        UserGroup {
            id: 1,
            subspace_id: Uint64::new(1),
            name: String::from("Test group"),
            description: String::from("Test group"),
            permissions: 1,
        }
    }

    pub fn get_mock_group_member() -> Addr {
        Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69")
    }

    pub fn get_mock_permission_detail() -> PermissionDetail {
        PermissionDetail::Group(GroupPermission {
            group_id: 1,
            permissions: 1,
        })
    }
}

pub struct MockSubspacesQuerier {}

impl MockSubspacesQuerier {
    pub fn custom_query_execute(query: SubspacesQueryRouter) -> ContractResult<Binary> {
        let response = match query.query_data {
            SubspacesQueryRoute::Subspaces { .. } => {
                let subspace = MockSubspacesQueries::get_mock_subspace();
                to_binary(&QuerySubspacesResponse {
                    subspaces: vec![subspace],
                    pagination: Default::default(),
                })
            }
            SubspacesQueryRoute::Subspace { .. } => {
                let subspace = MockSubspacesQueries::get_mock_subspace();
                to_binary(&QuerySubspaceResponse { subspace: subspace })
            }
            SubspacesQueryRoute::UserGroups { .. } => {
                let group = MockSubspacesQueries::get_mock_user_group();
                to_binary(&QueryUserGroupsResponse {
                    groups: vec![group],
                    pagination: Default::default(),
                })
            }
            SubspacesQueryRoute::UserGroup { .. } => {
                let group = MockSubspacesQueries::get_mock_user_group();
                to_binary(&QueryUserGroupResponse { group: group })
            }
            SubspacesQueryRoute::UserGroupMembers { .. } => {
                let member = MockSubspacesQueries::get_mock_group_member();
                to_binary(&QueryUserGroupMembersResponse {
                    members: vec![member],
                    pagination: Default::default(),
                })
            }
            SubspacesQueryRoute::UserPermissions { .. } => {
                let permission = MockSubspacesQueries::get_mock_permission_detail();
                to_binary(&QueryUserPermissionsResponse {
                    permissions: 1,
                    details: vec![permission],
                })
            }
        };
        response.into()
    }
}

pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<SubspacesQueryRouter>, impl CustomQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier =
        MockQuerier::<SubspacesQueryRouter>::new(&[(contract_addr, contract_balance)])
            .with_custom_handler(|query| {
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
        use std::ops::Deref;

        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(1).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        println!("response {:?}", response);
        assert_eq!(response, expected);
    }
}
