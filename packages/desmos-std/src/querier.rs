use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Uint64};

use crate::{
    query_router::{DesmosQuery, DesmosQueryRouter},
    subspaces::{
        query_router::SubspacesQuerier,
        query_routes::SubspacesRoutes,
        query_types::{
            QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
            QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
        },
    },
    types::{DesmosRoute, PageRequest},
};

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryRouter>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryRouter>) -> Self {
        DesmosQuerier { querier }
    }
}

impl<'a> SubspacesQuerier for DesmosQuerier<'a> {
    fn query_subspaces(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySubspacesResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::Subspaces {
                pagination: pagination,
            }),
        };
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_subspace(&self, subspace_id: Uint64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::Subspace {
                subspace_id: subspace_id,
            }),
        };
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_groups(
        &self,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::UserGroups {
                subspace_id,
                pagination,
            }),
        };
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::UserGroup {
                subspace_id,
                group_id,
            }),
        };
        let res: QueryUserGroupResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_group_members(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::UserGroupMembers {
                subspace_id,
                group_id,
                pagination,
            }),
        };
        let res: QueryUserGroupMembersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    fn query_user_permissions(
        &self,
        subspace_id: Uint64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = DesmosQueryRouter {
            route: DesmosRoute::Subspaces,
            query_data: DesmosQuery::Subspaces(SubspacesRoutes::UserPermissions {
                subspace_id,
                user,
            }),
        };
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
