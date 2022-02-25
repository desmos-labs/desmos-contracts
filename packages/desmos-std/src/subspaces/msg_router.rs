use cosmwasm_std::{Addr, CosmosMsg, CustomMsg, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    subspaces::{msg_routes::SubspacesMsgs},
    types::DesmosRoute,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubspacesMsgRouter {
    pub route: DesmosRoute,
    pub msg_data: SubspacesMsgs,
}

impl Into<CosmosMsg<SubspacesMsgRouter>> for SubspacesMsgRouter {
    fn into(self) -> CosmosMsg<SubspacesMsgRouter> {
        CosmosMsg::Custom(self)
    }
}
impl CustomMsg for SubspacesMsgRouter {}


pub struct SubspacesMsgBuilder{}
impl SubspacesMsgBuilder{
    pub fn new() -> Self {
        SubspacesMsgBuilder{}
    }
}

impl SubspacesMsgBuilder {
    pub fn create_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::CreateSubspace {
                name,
                description,
                treasury,
                owner,
                creator,
            },
        }
        .into()
    }

    pub fn edit_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::EditSubspace {
                name,
                description,
                treasury,
                owner,
                signer,
            },
        }
        .into()
    }

    pub fn delete_subspace(&self, subspace_id: Uint64, signer: Addr) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::DeleteSubspace {
                subspace_id,
                signer,
            },
        }
        .into()
    }

    pub fn create_user_group(
        &self,
        subspace_id: Uint64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::CreateUserGroup {
                subspace_id,
                name,
                description,
                default_permissions,
                creator,
            },
        }
        .into()
    }

    pub fn edit_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::EditUserGroup {
                subspace_id,
                group_id,
                name,
                description,
                signer,
            },
        }
        .into()
    }

    pub fn set_user_group_permissions(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::SetUserGroupPermissions {
                subspace_id,
                group_id,
                permissions,
                signer,
            },
        }
        .into()
    }

    pub fn delete_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::DeleteUserGroup {
                subspace_id,
                group_id,
                signer,
            },
        }
        .into()
    }

    pub fn add_user_to_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::AddUserToUserGroup {
                subspace_id,
                group_id,
                user,
                signer,
            },
        }
        .into()
    }

    pub fn remove_user_from_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::RemoveUserFromUserGroup {
                subspace_id,
                group_id,
                user,
                signer,
            },
        }
        .into()
    }

    pub fn set_user_permissions(
        &self,
        subspace_id: Uint64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<SubspacesMsgRouter> {
        SubspacesMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: SubspacesMsgs::SetUserPermissions {
                subspace_id,
                user,
                permissions,
                signer,
            },
        }
        .into()
    }
}
