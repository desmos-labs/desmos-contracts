use crate::profiles::msg::ProfilesMsg;
use cosmwasm_std::Addr;

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
    ) -> ProfilesMsg {
        ProfilesMsg::SaveProfile {
            dtag,
            nickname,
            bio,
            profile_picture,
            cover_picture,
            creator,
        }
    }

    pub fn delete_profile(&self, creator: Addr) -> ProfilesMsg {
        ProfilesMsg::DeleteProfile { creator }
    }

    pub fn request_dtag_transfer(&self, sender: Addr, receiver: Addr) -> ProfilesMsg {
        ProfilesMsg::RequestDtagTransfer { receiver, sender }
    }

    pub fn accept_dtag_transfer_request(
        &self,
        new_dtag: String,
        sender: Addr,
        receiver: Addr,
    ) -> ProfilesMsg {
        ProfilesMsg::AcceptDtagTransferRequest {
            new_dtag,
            sender,
            receiver,
        }
    }

    pub fn refuse_dtag_transfer_request(&self, sender: Addr, receiver: Addr) -> ProfilesMsg {
        ProfilesMsg::RefuseDtagTransferRequest { sender, receiver }
    }

    pub fn cancel_dtag_transfer_request(&self, receiver: Addr, sender: Addr) -> ProfilesMsg {
        ProfilesMsg::CancelDtagTransferRequest { receiver, sender }
    }

    pub fn create_relationship(
        &self,
        sender: Addr,
        receiver: Addr,
        subspace: String,
    ) -> ProfilesMsg {
        ProfilesMsg::CreateRelationship {
            sender,
            receiver,
            subspace,
        }
    }

    pub fn delete_relationship(
        &self,
        user: Addr,
        counterparty: Addr,
        subspace: String,
    ) -> ProfilesMsg {
        ProfilesMsg::DeleteRelationship {
            user,
            counterparty,
            subspace,
        }
    }

    pub fn block_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace: String,
    ) -> ProfilesMsg {
        ProfilesMsg::BlockUser {
            blocker,
            blocked,
            reason,
            subspace,
        }
    }

    pub fn unblock_user(&self, blocker: Addr, blocked: Addr, subspace: String) -> ProfilesMsg {
        ProfilesMsg::UnblockUser {
            blocker,
            blocked,
            subspace,
        }
    }
}
