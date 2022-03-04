use cosmwasm_std::Addr;

use crate::subspaces::msg::SubspacesMsg;

pub struct SubspacesMsgBuilder {}
impl SubspacesMsgBuilder {
    pub fn new() -> Self {
        SubspacesMsgBuilder {}
    }
}

impl SubspacesMsgBuilder {
    pub fn create_subspace(
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateSubspace {
            name,
            description,
            treasury,
            owner,
            creator,
        }
    }

    pub fn edit_subspace(
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditSubspace {
            name,
            description,
            treasury,
            owner,
            signer,
        }
    }

    pub fn delete_subspace(&self, subspace_id: u64, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteSubspace {
            subspace_id,
            signer,
        }
    }

    pub fn create_user_group(
        &self,
        subspace_id: u64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateUserGroup {
            subspace_id,
            name,
            description,
            default_permissions,
            creator,
        }
    }

    pub fn edit_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditUserGroup {
            subspace_id,
            group_id,
            name,
            description,
            signer,
        }
    }

    pub fn set_user_group_permissions(
        &self,
        subspace_id: u64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserGroupPermissions {
            subspace_id,
            group_id,
            permissions,
            signer,
        }
    }

    pub fn delete_user_group(&self, subspace_id: u64, group_id: u32, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteUserGroup {
            subspace_id,
            group_id,
            signer,
        }
    }

    pub fn add_user_to_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::AddUserToUserGroup {
            subspace_id,
            group_id,
            user,
            signer,
        }
    }

    pub fn remove_user_from_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id,
            group_id,
            user,
            signer,
        }
    }

    pub fn set_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserPermissions {
            subspace_id,
            user,
            permissions,
            signer,
        }
    }
}
