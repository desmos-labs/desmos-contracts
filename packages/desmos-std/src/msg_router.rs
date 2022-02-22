use crate::profiles::msg_routes::ProfilesMsgs;
use crate::profiles::msg_routes::ProfilesMsgs::{
    AcceptDtagTransferRequest, BlockUser, CancelDtagTransferRequest, CreateRelationship,
    DeleteProfile, DeleteRelationship, RefuseDtagTransferRequest, RequestDtagTransfer, SaveProfile,
    UnblockUser,
};
use crate::types::DesmosRoute;
use cosmwasm_std::{Addr, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgRouter {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsg,
}

impl Into<CosmosMsg<DesmosMsgRouter>> for DesmosMsgRouter {
    fn into(self) -> CosmosMsg<DesmosMsgRouter> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for DesmosMsgRouter {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive] // missing chain-links and app-links messages (not necessary to me)
pub enum DesmosMsg {
    Profiles(ProfilesMsgs),
}

pub fn save_profile(
    dtag: String,
    creator: Addr,
    nickname: Option<String>,
    bio: Option<String>,
    profile_picture: Option<String>,
    cover_picture: Option<String>,
) -> CosmosMsg<DesmosMsgRouter> {
    // try to unwrap all the optional fields
    let nickname = nickname.unwrap_or_default();
    let bio = bio.unwrap_or_default();
    let profile_picture = profile_picture.unwrap_or_default();
    let cover_picture = cover_picture.unwrap_or_default();

    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(SaveProfile {
            dtag,
            nickname,
            bio,
            profile_picture,
            cover_picture,
            creator,
        }),
    }
    .into()
}

pub fn delete_profile(creator: Addr) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(DeleteProfile { creator }),
    }
    .into()
}

pub fn request_dtag_transfer(sender: Addr, receiver: Addr) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(RequestDtagTransfer { receiver, sender }),
    }
    .into()
}

pub fn accept_dtag_transfer_request(
    new_dtag: String,
    sender: Addr,
    receiver: Addr,
) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(AcceptDtagTransferRequest {
            new_dtag,
            sender,
            receiver,
        }),
    }
    .into()
}

pub fn refuse_dtag_transfer_request(sender: Addr, receiver: Addr) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(RefuseDtagTransferRequest { sender, receiver }),
    }
    .into()
}

pub fn cancel_dtag_transfer_request(receiver: Addr, sender: Addr) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(CancelDtagTransferRequest { receiver, sender }),
    }
    .into()
}

pub fn create_relationship(
    sender: Addr,
    receiver: Addr,
    subspace: String,
) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(CreateRelationship {
            sender,
            receiver,
            subspace,
        }),
    }
    .into()
}

pub fn delete_relationship(
    user: Addr,
    counterparty: Addr,
    subspace: String,
) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(DeleteRelationship {
            user,
            counterparty,
            subspace,
        }),
    }
    .into()
}

pub fn block_user(
    blocker: Addr,
    blocked: Addr,
    reason: String,
    subspace: String,
) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(BlockUser {
            blocker,
            blocked,
            reason,
            subspace,
        }),
    }
    .into()
}

pub fn unblock_user(blocker: Addr, blocked: Addr, subspace: String) -> CosmosMsg<DesmosMsgRouter> {
    DesmosMsgRouter {
        route: DesmosRoute::Profiles,
        msg_data: DesmosMsg::Profiles(UnblockUser {
            blocker,
            blocked,
            subspace,
        }),
    }
    .into()
}
