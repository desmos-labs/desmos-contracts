use cosmwasm_std::{Addr, CosmosMsg};

pub trait ProfilesMsgRouter<T> {
    fn save_profile(
        &self,
        dtag: String,
        creator: Addr,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
    ) -> CosmosMsg<T>;
    fn delete_profile(&self, creator: Addr) -> CosmosMsg<T>;
    fn request_dtag_transfer(&self, sender: Addr, receiver: Addr) -> CosmosMsg<T>;
    fn accept_dtag_transfer_request(&self, new_dtag: String, sender: Addr, receiver: Addr)
        -> CosmosMsg<T>;
    fn refuse_dtag_transfer_request(&self, sender: Addr, receiver: Addr) -> CosmosMsg<T>;
    fn cancel_dtag_transfer_request(&self, receiver: Addr, sender: Addr) -> CosmosMsg<T>;
    fn create_relationship(&self, sender: Addr, receiver: Addr, subspace: String) -> CosmosMsg<T>;
    fn delete_relationship(&self, user: Addr, counterparty: Addr, subspace: String) -> CosmosMsg<T>;
    fn block_user(&self, blocker: Addr, blocked: Addr, reason: String, subspace: String) -> CosmosMsg<T>;
    fn unblock_user(&self, blocker: Addr, blocked: Addr, subspace: String) -> CosmosMsg<T>;
}
