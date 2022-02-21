use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::subspaces::models::{Subspace, UserGroup, PermissionDetail};
use crate::types::{PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspacesResponse {
    pub subspaces : Vec<Subspace>,
    pub pagination: PageResponse
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspaceResponse {
    pub subspace : Subspace
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupsResponse {
    pub groups : Vec<UserGroup>,
    pub pagination: PageResponse
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupResponse{
    pub group : UserGroup
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupMembersResponse {
    pub members : Vec<String>,
    pub pagination : PageResponse
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserPermissionsResponse {
    pub permissions : u32,
    pub details : Vec<PermissionDetail>
}