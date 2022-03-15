use crate::types::PageRequest;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesQuery {
    Profile {
        user: Addr,
    },
    IncomingDtagTransferRequests {
        receiver: Addr,
        pagination: Option<PageRequest>,
    },
    ChainLinks {
        user: Addr,
        chain_name: String,
        target: String,
        pagination: Option<PageRequest>,
    },
    AppLinks {
        user: Addr,
        application: String,
        username: String,
        pagination: Option<PageRequest>,
    },
    ApplicationLinkByChainID {
        client_id: String,
    },
}
