use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsMsg {
    CreateRelationship {
        sender: Addr,
        receiver: Addr,
        subspace: String,
    },
    DeleteRelationship {
        user: Addr,
        counterparty: Addr,
        subspace: String,
    },
    BlockUser {
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace: String,
    },
    UnblockUser {
        blocker: Addr,
        blocked: Addr,
        subspace: String,
    },
}
