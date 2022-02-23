use cosmwasm_std::{Addr, CosmosMsg};

pub trait SubspacesMsgRouter<T> {
    fn create_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> CosmosMsg<T>;

    fn edit_subspace(&self,name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,) -> CosmosMsg<T>;

    fn delete_subspace(&self, subspace_id: u64, signer: Addr) -> CosmosMsg<T>;

    fn create_user_group(
        &self,
        subspace_id: u64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> CosmosMsg<T>;

    fn edit_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> CosmosMsg<T>;

    fn set_user_group_permissions(
        &self,
        subspace_id: u64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<T>;

    fn delete_user_group(&self, subspace_id: u64, group_id: u32, signer: Addr) -> CosmosMsg<T>;

    fn add_user_to_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<T>;

    fn remove_user_from_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<T>;
    fn set_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<T>;
}
