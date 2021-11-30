use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CosmosMsg, CustomMsg};
use crate::types::DesmosRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgWrapper {
    pub route: DesmosRoute,
    pub msg: DesmosMsg,
}

impl Into<CosmosMsg<DesmosMsgWrapper>> for DesmosMsgWrapper {
    fn into(self) -> CosmosMsg<DesmosMsgWrapper> { CosmosMsg::Custom(self) }
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
        creator: String
    },
    DeleteProfile {
        creator: String,
    },
    RequestDTagTransfer {
      receiver: String,
      sender: String,
    },
    CancelDTagTransferRequest {
      receiver: String,
      sender: String,
    },
    AcceptDTagTransferRequest {
      new_dtag: String,
      sender: String,
      receiver: String,
    },
    RefuseDTagTransferRequest {
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
