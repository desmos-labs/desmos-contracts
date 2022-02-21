use crate::types::{DesmosRoute, PageRequest};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DesmosQueryRouter is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryRouter {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery,
}

impl CustomQuery for DesmosQueryRouter {}

/// DesmosQuery represents the available desmos network queries
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Subspaces(SubspacesQuery)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesQuery {
    Subspaces {
        pagination : Option<PageRequest>
    },
    Subspace {
        subspace_id : u64
    },
    UserGroups {
        subspace_id : u64,
        pagination : Option<PageRequest>
    },
    UserGroup {
        subspace_id : u64,
        group_id : u32
    },
    UserGroupMembers {
        subspace_id : u64,
        group_id : u32
    },
    UserPermissions {
        subspace_id : u64,
        user : String
    }
}