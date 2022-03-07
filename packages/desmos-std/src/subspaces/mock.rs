use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

use crate::query::{DesmosQuery, DesmosQueryRouter};
use crate::subspaces::{
    models::{GroupPermission, PermissionDetail, Subspace, UserGroup},
    query_router::{SubspacesQueryRoute},
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

pub struct MockSubspacesQuerier;

impl MockSubspacesQuerier {
    pub fn query(query: &DesmosQuery) -> ContractResult<Binary> {
        let response = match query.query_data {
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::Subspaces { .. }) => {
                let subspace = MockSubspacesQueries::get_mock_subspace();
                to_binary(&QuerySubspacesResponse {
                    subspaces: vec![subspace],
                    pagination: Default::default(),
                })
            }
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::Subspace { .. }) => {
                let subspace = MockSubspacesQueries::get_mock_subspace();
                to_binary(&QuerySubspaceResponse { subspace: subspace })
            }
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::UserGroups { .. }) => {
                let group = MockSubspacesQueries::get_mock_user_group();
                to_binary(&QueryUserGroupsResponse {
                    groups: vec![group],
                    pagination: Default::default(),
                })
            }
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::UserGroup { .. }) => {
                let group = MockSubspacesQueries::get_mock_user_group();
                to_binary(&QueryUserGroupResponse { group: group })
            }
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::UserGroupMembers { .. }) => {
                let member = MockSubspacesQueries::get_mock_group_member();
                to_binary(&QueryUserGroupMembersResponse {
                    members: vec![member],
                    pagination: Default::default(),
                })
            }
            DesmosQueryRouter::Subspaces(SubspacesQueryRoute::UserPermissions { .. }) => {
                let permission = MockSubspacesQueries::get_mock_permission_detail();
                to_binary(&QueryUserPermissionsResponse {
                    permissions: Default::default(),
                    details: vec![permission],
                })
            }
        };
       response.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_subspaces() {
        let route = SubspacesQueryRoute::Subspaces{ pagination: Default::default() };
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QuerySubspacesResponse {
            subspaces: vec![MockSubspacesQueries::get_mock_subspace()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_subspace() {
        let route = SubspacesQueryRoute::Subspace{ subspace_id: 1};
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_groups() {
        let route = SubspacesQueryRoute::UserGroups{ subspace_id: 1, pagination: Default::default() };
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QueryUserGroupsResponse {
            groups: vec![MockSubspacesQueries::get_mock_user_group()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_group() {
        let route = SubspacesQueryRoute::UserGroup{ subspace_id: 1, group_id: 1 };
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QueryUserGroupResponse {
            group: MockSubspacesQueries::get_mock_user_group(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_group_members() {
        let route = SubspacesQueryRoute::UserGroupMembers{ subspace_id: 1, group_id: 1, pagination: Default::default() };
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QueryUserGroupMembersResponse {
            members: vec![MockSubspacesQueries::get_mock_group_member()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_permissions() {
        let route = SubspacesQueryRoute::UserPermissions{ subspace_id: 1, user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69") };
        let response = MockSubspacesQuerier::query(&DesmosQuery::from(route));
        let expected = to_binary(&QueryUserPermissionsResponse {
            permissions: Default::default(),
            details: vec![MockSubspacesQueries::get_mock_permission_detail()],
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }
}