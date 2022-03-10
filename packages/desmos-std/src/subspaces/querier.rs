use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult, Uint64};

use crate::{
    query::DesmosQuery,
    subspaces::{
        query::SubspacesQuery,
        query_types::{
            QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
            QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
        },
    },
    types::PageRequest,
};

pub struct SubspacesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> SubspacesQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }
}

impl<'a> SubspacesQuerier<'a> {
    pub fn query_subspaces(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySubspacesResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Subspaces { pagination });
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_subspace(&self, subspace_id: Uint64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Subspace { subspace_id });
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_groups(
        &self,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroups {
            subspace_id,
            pagination,
        });
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroup {
            subspace_id,
            group_id,
        });
        let res: QueryUserGroupResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group_members(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroupMembers {
            subspace_id,
            group_id,
            pagination,
        });
        let res: QueryUserGroupMembersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_permissions(
        &self,
        subspace_id: Uint64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserPermissions { subspace_id, user });
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::mock::mock_dependencies_with_custom_querier;
    use crate::subspaces::mock::MockSubspacesQueries;

    #[test]
    fn test_query_subspaces() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspaces(Default::default());
        let expected = QuerySubspacesResponse {
            subspaces: vec![MockSubspacesQueries::get_mock_subspace()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_subspace() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(Uint64::new(1));
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_groups() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_groups(Uint64::new(1), Default::default());
        let expected = QueryUserGroupsResponse {
            groups: vec![MockSubspacesQueries::get_mock_user_group()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_group() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_group(Uint64::new(1), 1);
        let expected = QueryUserGroupResponse {
            group: MockSubspacesQueries::get_mock_user_group(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_group_members() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_group_members(Uint64::new(1), 1, Default::default());
        let expected = QueryUserGroupMembersResponse {
            members: vec![MockSubspacesQueries::get_mock_group_member()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_permissions() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_permissions(
            Uint64::new(1),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = QueryUserPermissionsResponse {
            permissions: Default::default(),
            details: vec![MockSubspacesQueries::get_mock_permission_detail()],
        };
        assert_eq!(response.ok(), Some(expected));
    }
}
