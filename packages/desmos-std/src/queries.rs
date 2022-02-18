use crate::types::{DesmosRoute, PageRequest};
use cosmwasm_std::{Addr, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DesmosQueryWrapper is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryWrapper {
    pub route: DesmosRoute,
    pub query_data: DesmosQuery,
}

impl CustomQuery for DesmosQueryWrapper {}

/// DesmosQuery represents the available desmos network queries
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    Profile {
        user: Addr
    },
    IncomingDtagTransferRequests {
        receiver: Addr,
        pagination: Option<PageRequest>
    },
    Relationships {
        user: Addr,
        subspace_id: u64,
        pagination: Option<PageRequest>
    },
    Blocks {
        user: Addr,
        subspace_id: u64,
        pagination: Option<PageRequest>
    },
    ChainLinks {
        user: Addr,
        pagination: Option<PageRequest>
    },
    UserChainLink {
        user: Addr,
        chain_name: String,
        target: String,
    },
    AppLinks {
        user: Addr,
        pagination: Option<PageRequest>
    },
    UserAppLinks {
        user: Addr,
        application: String,
        username: String
    },
    ApplicationLinkByChainID {
        client_id: String,
    }
}
