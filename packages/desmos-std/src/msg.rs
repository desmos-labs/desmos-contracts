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

#[allow(clippy::all)]
impl Into<CosmosMsg<DesmosMsgWrapper>> for DesmosMsgWrapper {
    fn into(self) -> CosmosMsg<DesmosMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for DesmosMsgWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive] // missing chain-links and app-links messages
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
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: String,
        receiver: String,
    },
    RefuseDtagTransferRequest {
        sender: String,
        receiver: String,
    },
    CancelDtagTransferRequest {
        receiver: String,
        sender: String,
    },
    CreateRelationship {
        sender: String,
        receiver: String,
        subspace: String,
    },
    DeleteRelationship {
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

pub fn save_profile(
    dtag: String,
    creator: String,
    nickname: Option<String>,
    bio: Option<String>,
    profile_picture: Option<String>,
    cover_picture: Option<String>,
) -> CosmosMsg<DesmosMsgWrapper> {
    // try to unwrap all the optional fields
    let nickname = nickname.unwrap_or_default();
    let bio = bio.unwrap_or_default();
    let profile_picture = profile_picture.unwrap_or_default();
    let cover_picture = cover_picture.unwrap_or_default();

    DesmosMsgWrapper {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::SaveProfile {
            dtag,
            nickname,
            bio,
            profile_picture,
            cover_picture,
            creator,
        },
    }
    .into()
}

pub fn request_dtag_transfer(sender: String, receiver: String) -> CosmosMsg<DesmosMsgWrapper> {
    DesmosMsgWrapper {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::RequestDtagTransfer { receiver, sender },
    }
    .into()
}
