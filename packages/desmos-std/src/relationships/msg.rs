use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsMsg {
    CreateRelationship {
        signer: Addr,
        counterparty: Addr,
        subspace_id: Uint64,
    },
    DeleteRelationship {
        signer: Addr,
        counterparty: Addr,
        subspace_id: Uint64,
    },
    BlockUser {
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace_id: Uint64,
    },
    UnblockUser {
        blocker: Addr,
        blocked: Addr,
        subspace_id: Uint64,
    },
}
