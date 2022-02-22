use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::types::PageRequest;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesRoutes {
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
    },
}
