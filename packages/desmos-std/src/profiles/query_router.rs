use crate::types::{DesmosRoute, PageRequest};
use cosmwasm_std::{Addr, CustomQuery, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DesmosQueryRouter is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfilesQueryRouter {
    pub route: DesmosRoute,
    pub query_data: ProfilesQueryRoute,
}

impl CustomQuery for ProfilesQueryRouter {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesQueryRoute {
    Profile {
        user: Addr,
    },
    IncomingDtagTransferRequests {
        receiver: Addr,
        pagination: Option<PageRequest>,
    },
    Relationships {
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    Blocks {
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    ChainLinks {
        user: Addr,
        pagination: Option<PageRequest>,
    },
    UserChainLink {
        user: Addr,
        chain_name: String,
        target: String,
    },
    AppLinks {
        user: Addr,
        pagination: Option<PageRequest>,
    },
    UserAppLinks {
        user: Addr,
        application: String,
        username: String,
    },
    ApplicationLinkByChainID {
        client_id: String,
    },
}
