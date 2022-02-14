use crate::types::{DesmosRoute, PageRequest};
use cosmwasm_std::CustomQuery;
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
    Profile{
        user: String
    },
    IncomingDtagTransferRequests {
        receiver: String,
        pagination: Option<PageRequest>
    },
    Relationships {
        user: String,
        subspace_id: String,
        pagination: Option<PageRequest>
    },
    Blocks {
        user: String,
        subspace_id: String,
        pagination: Option<PageRequest>
    },
    ChainLinks {
        user: String,
        pagination: Option<PageRequest>
    },
    UserChainLink {
        user: String,
        chain_name: String,
        target: String,
    },
    AppLinks {
        user: String,
        pagination: Option<PageRequest>
    },
    UserAppLinks {
        user: String,
        application: String,
        username: String
    },
    ApplicationLinkByChainID {
        client_id: String,
    }
}
