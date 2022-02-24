use crate::{
    profiles::{
        msg_router::ProfilesMsgRouter,
        msg_routes::ProfilesMsgs,
        msg_routes::ProfilesMsgs::{
            AcceptDtagTransferRequest, BlockUser, CancelDtagTransferRequest, CreateRelationship,
            DeleteProfile, DeleteRelationship, RefuseDtagTransferRequest, RequestDtagTransfer,
            SaveProfile, UnblockUser,
        },
    },
    types::DesmosRoute,
};
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

pub struct DesmosMsgBuilder {}
impl DesmosMsgBuilder {
    pub fn new() -> Self {
        DesmosMsgBuilder {}
    }
}

impl ProfilesMsgRouter<DesmosMsgRouter> for DesmosMsgBuilder {
    fn save_profile(
        &self,
        dtag: String,
        creator: Addr,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
    ) -> CosmosMsg<DesmosMsgRouter> {
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

    fn delete_profile(&self, creator: Addr) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsg::Profiles(DeleteProfile { creator }),
        }
        .into()
    }

    fn request_dtag_transfer(&self, sender: Addr, receiver: Addr) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsg::Profiles(RequestDtagTransfer { receiver, sender }),
        }
        .into()
    }

    fn accept_dtag_transfer_request(
        &self,
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

    fn refuse_dtag_transfer_request(&self, sender: Addr, receiver: Addr) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsg::Profiles(RefuseDtagTransferRequest { sender, receiver }),
        }
        .into()
    }

    fn cancel_dtag_transfer_request(&self,receiver: Addr, sender: Addr) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsg::Profiles(CancelDtagTransferRequest { receiver, sender }),
        }
        .into()
    }

    fn create_relationship(
        &self,
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

    fn delete_relationship(
        &self,
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

    fn block_user(
        &self,
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

    fn unblock_user(&self, blocker: Addr, blocked: Addr, subspace: String) -> CosmosMsg<DesmosMsgRouter> {
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
}
