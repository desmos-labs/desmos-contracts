use crate::types::DesmosRoute;
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgWrapper {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsg,
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

pub fn save_profile(dtag: String, creator: String) -> CosmosMsg<DesmosMsgWrapper> {
    DesmosMsgWrapper {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::SaveProfile {
            dtag,
            nickname: "".to_string(),
            bio: "".to_string(),
            profile_picture: "".to_string(),
            cover_picture: "".to_string(),
            creator
        }
    }.into()
}

pub fn request_dtag_transfer(sender: String, receiver: String) -> CosmosMsg<DesmosMsgWrapper> {
    DesmosMsgWrapper{
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::RequestDtagTransfer { receiver, sender }
    }.into()
}
