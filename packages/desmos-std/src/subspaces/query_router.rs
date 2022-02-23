use crate::{
    subspaces::query_types::{
        QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
        QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
    },
    types::PageRequest,
};
use cosmwasm_std::{Addr, StdResult};

pub trait SubspacesQuerier {
    fn query_subspaces(&self, pagination: Option<PageRequest>)
        -> StdResult<QuerySubspacesResponse>;
    fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse>;
    fn query_user_groups(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse>;
    fn query_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse>;
    fn query_user_group_members(
        &self,
        subspace_id: u64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse>;
    fn query_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse>;
}