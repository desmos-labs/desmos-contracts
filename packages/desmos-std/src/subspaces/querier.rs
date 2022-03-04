use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};

use crate::{
    query::DesmosQuery,
    types::{PageRequest},
    subspaces::{
        query_router::SubspacesQueryRoute,
        query_types::{
            QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
            QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
        },
    },
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
        let request = DesmosQuery::from(SubspacesQueryRoute::Subspaces {
            pagination: pagination,
        });
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQuery::from(SubspacesQueryRoute::Subspace {
            subspace_id: subspace_id,
        });
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_groups(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = DesmosQuery::from(SubspacesQueryRoute::UserGroups {
            subspace_id,
            pagination,
        });
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse> {
        let request = DesmosQuery::from(SubspacesQueryRoute::UserGroup {
            subspace_id,
            group_id,
        });
        let res: QueryUserGroupResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group_members(
        &self,
        subspace_id: u64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse> {
        let request = DesmosQuery::from(SubspacesQueryRoute::UserGroupMembers {
            subspace_id,
            group_id,
            pagination,
        });
        let res: QueryUserGroupMembersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = DesmosQuery::from(SubspacesQueryRoute::UserPermissions { subspace_id, user });
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
