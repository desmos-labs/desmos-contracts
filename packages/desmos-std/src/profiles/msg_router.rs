use crate::{profiles::msg_routes::ProfilesMsgs, types::DesmosRoute};
use cosmwasm_std::{Addr, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfilesMsgRouter {
    pub route: DesmosRoute,
    pub msg_data: ProfilesMsgs,
}

impl Into<CosmosMsg<ProfilesMsgRouter>> for ProfilesMsgRouter {
    fn into(self) -> CosmosMsg<ProfilesMsgRouter> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for ProfilesMsgRouter {}

pub struct ProfilesMsgBuilder {}

impl ProfilesMsgBuilder {
    pub fn new() -> Self {
        ProfilesMsgBuilder {}
    }
}

impl Default for ProfilesMsgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfilesMsgBuilder {
    pub fn save_profile(
        &self,
        dtag: String,
        creator: Addr,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::SaveProfile {
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

    pub fn delete_profile(&self, creator: Addr) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::DeleteProfile { creator },
        }
        .into()
    }

    pub fn request_dtag_transfer(
        &self,
        sender: Addr,
        receiver: Addr,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::RequestDtagTransfer { receiver, sender },
        }
        .into()
    }

    pub fn accept_dtag_transfer_request(
        &self,
        new_dtag: String,
        sender: Addr,
        receiver: Addr,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::AcceptDtagTransferRequest {
                new_dtag,
                sender,
                receiver,
            },
        }
        .into()
    }

    pub fn refuse_dtag_transfer_request(
        &self,
        sender: Addr,
        receiver: Addr,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::RefuseDtagTransferRequest { sender, receiver },
        }
        .into()
    }

    pub fn cancel_dtag_transfer_request(
        &self,
        receiver: Addr,
        sender: Addr,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::CancelDtagTransferRequest { receiver, sender },
        }
        .into()
    }

    pub fn create_relationship(
        &self,
        sender: Addr,
        receiver: Addr,
        subspace: String,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::CreateRelationship {
                sender,
                receiver,
                subspace,
            },
        }
        .into()
    }

    pub fn delete_relationship(
        &self,
        user: Addr,
        counterparty: Addr,
        subspace: String,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::DeleteRelationship {
                user,
                counterparty,
                subspace,
            },
        }
        .into()
    }

    pub fn block_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace: String,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::BlockUser {
                blocker,
                blocked,
                reason,
                subspace,
            },
        }
        .into()
    }

    pub fn unblock_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        subspace: String,
    ) -> CosmosMsg<ProfilesMsgRouter> {
        ProfilesMsgRouter {
            route: DesmosRoute::Profiles,
            msg_data: ProfilesMsgs::UnblockUser {
                blocker,
                blocked,
                subspace,
            },
        }
        .into()
    }
}
