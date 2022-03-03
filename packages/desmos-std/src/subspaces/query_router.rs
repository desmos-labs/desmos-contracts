use cosmwasm_std::{Addr, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{DesmosRoute, PageRequest};

/// SubspacesQueryRouter is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubspacesQueryRouter {
    pub route: DesmosRoute,
    pub query_data: SubspacesQueryRoute,
}

impl CustomQuery for SubspacesQueryRouter {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesQueryRoute {
    Subspaces {
        pagination: Option<PageRequest>,
    },
    Subspace {
        subspace_id: u64,
    },
    UserGroups {
        subspace_id: u64,
        pagination: Option<PageRequest>,
    },
    UserGroup {
        subspace_id: u64,
        group_id: u32,
    },
    UserGroupMembers {
        subspace_id: u64,
        group_id: u32,
        pagination: Option<PageRequest>,
    },
    UserPermissions {
        subspace_id: u64,
        user: Addr,
    },
}
