use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Uint64, Querier};

use crate::{
    subspaces::{
        query_router::{SubspacesQueryRouter, SubspacesQueryRoute},
        query_types::{
            QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
            QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
        },
    },
    types::{DesmosRoute, PageRequest},
};

pub struct SubspacesQuerier<'a> {
    querier: QuerierWrapper<'a, SubspacesQueryRouter>,
}

impl <'a> SubspacesQuerier<'a> {
    pub fn new(querier:  &'a dyn Querier) -> Self {
        Self { querier: QuerierWrapper::<'a, SubspacesQueryRouter>::new(querier) }
    }
}

impl<'a> SubspacesQuerier<'a> {
    pub fn query_subspaces(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySubspacesResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::Subspaces {
                pagination: pagination,
            },
        };
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_subspace(&self, subspace_id: Uint64) -> StdResult<QuerySubspaceResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::Subspace {
                subspace_id: subspace_id,
            },
        };
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_groups(
        &self,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::UserGroups {
                subspace_id,
                pagination,
            },
        };
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::UserGroup {
                subspace_id,
                group_id,
            },
        };
        let res: QueryUserGroupResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group_members(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::UserGroupMembers {
                subspace_id,
                group_id,
                pagination,
            },
        };
        let res: QueryUserGroupMembersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_permissions(
        &self,
        subspace_id: Uint64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = SubspacesQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: SubspacesQueryRoute::UserPermissions {
                subspace_id,
                user,
            },
        };
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
