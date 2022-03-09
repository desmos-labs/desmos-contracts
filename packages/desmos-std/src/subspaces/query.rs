use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::PageRequest;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesQuery {
    Subspaces {
        pagination: Option<PageRequest>,
    },
    Subspace {
        subspace_id: Uint64,
    },
    UserGroups {
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    UserGroup {
        subspace_id: Uint64,
        group_id: u32,
    },
    UserGroupMembers {
        subspace_id: Uint64,
        group_id: u32,
        pagination: Option<PageRequest>,
    },
    UserPermissions {
        subspace_id: Uint64,
        user: Addr,
    },
}
