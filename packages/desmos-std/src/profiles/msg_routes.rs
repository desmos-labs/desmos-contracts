use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesMsgs {
    SaveProfile {
        dtag: String,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
        creator: Addr,
    },
    DeleteProfile {
        creator: Addr,
    },
    RequestDtagTransfer {
        receiver: Addr,
        sender: Addr,
    },
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: Addr,
        receiver: Addr,
    },
    RefuseDtagTransferRequest {
        sender: Addr,
        receiver: Addr,
    },
    CancelDtagTransferRequest {
        receiver: Addr,
        sender: Addr,
    },
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
