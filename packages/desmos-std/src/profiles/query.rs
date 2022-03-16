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
        user: Option<Addr>,
        chain_name: Option<String>,
        target: Option<String>,
        pagination: Option<PageRequest>,
    },
    AppLinks {
        user: Option<Addr>,
        application: Option<String>,
        username: Option<String>,
        pagination: Option<PageRequest>,
    },
    ApplicationLinkByChainID {
        client_id: String,
    },
}
