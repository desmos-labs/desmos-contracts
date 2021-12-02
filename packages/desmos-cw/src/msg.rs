use crate::types::DesmosRoute;
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgWrapper {
    pub route: DesmosRoute,
    pub msg: DesmosMsg,
}

impl Into<CosmosMsg<DesmosMsgWrapper>> for DesmosMsgWrapper {
    fn into(self) -> CosmosMsg<DesmosMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for DesmosMsgWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DesmosMsg {
    SaveProfile {
        dtag: String,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
        creator: String,
    },
    DeleteProfile {
        creator: String,
    },
    RequestDtagTransfer {
        receiver: String,
        sender: String,
    },
    CancelDtagTransferRequest {
        receiver: String,
        sender: String,
    },
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: String,
        receiver: String,
    },
    RefuseDtagTransferRequest {
        sender: String,
        receiver: String,
    },
    CreateRelationship {
        sender: String,
        receiver: String,
        subspace: String,
    },
    DeleteRelationships {
        user: String,
        counterparty: String,
        subspace: String,
    },
    BlockUser {
        blocker: String,
        blocked: String,
        reason: String,
        subspace: String,
    },
    UnblockUser {
        blocker: String,
        blocked: String,
        subspace: String,
    },
}
