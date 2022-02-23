use cosmwasm_std::{Addr, CosmosMsg};

pub trait ProfilesMsgRouter<T> {
    fn save_profile(
        dtag: String,
        creator: Addr,
        nickname: Option<String>,
        bio: Option<String>,
        profile_picture: Option<String>,
        cover_picture: Option<String>,
    ) -> CosmosMsg<T>;
    fn delete_profile(creator: Addr) -> CosmosMsg<T>;
    fn request_dtag_transfer(sender: Addr, receiver: Addr) -> CosmosMsg<T>;
    fn accept_dtag_transfer_request(new_dtag: String, sender: Addr, receiver: Addr)
        -> CosmosMsg<T>;
    fn refuse_dtag_transfer_request(sender: Addr, receiver: Addr) -> CosmosMsg<T>;
    fn cancel_dtag_transfer_request(receiver: Addr, sender: Addr) -> CosmosMsg<T>;
    fn create_relationship(sender: Addr, receiver: Addr, subspace: String) -> CosmosMsg<T>;
    fn delete_relationship(user: Addr, counterparty: Addr, subspace: String) -> CosmosMsg<T>;
    fn block_user(blocker: Addr, blocked: Addr, reason: String, subspace: String) -> CosmosMsg<T>;
    fn unblock_user(blocker: Addr, blocked: Addr, subspace: String) -> CosmosMsg<T>;
}
