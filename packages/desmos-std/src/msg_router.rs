use crate::{
    subspaces::{msg_builder::SubspacesMsgBuilder, msg_routes::SubspacesMsgs},
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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMsg {
    Subspaces(SubspacesMsgs),
}

impl Into<CosmosMsg<DesmosMsgRouter>> for DesmosMsgRouter {
    fn into(self) -> CosmosMsg<DesmosMsgRouter> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for DesmosMsgRouter {}

pub struct DesmosMsgBuilder {}
impl DesmosMsgBuilder{
    pub fn new() -> Self {
        DesmosMsgBuilder{}
    }
}

impl SubspacesMsgBuilder<DesmosMsgRouter> for DesmosMsgBuilder {
    fn create_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::CreateSubspace {
                name,
                description,
                treasury,
                owner,
                creator,
            }),
        }
        .into()
    }

    fn edit_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::EditSubspace {
                name,
                description,
                treasury,
                owner,
                signer,
            }),
        }
        .into()
    }

    fn delete_subspace(&self, subspace_id: u64, signer: Addr) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::DeleteSubspace {
                subspace_id,
                signer,
            }),
        }
        .into()
    }

    fn create_user_group(
        &self,
        subspace_id: u64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::CreateUserGroup {
                subspace_id,
                name,
                description,
                default_permissions,
                creator,
            }),
        }
        .into()
    }

    fn edit_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::EditUserGroup {
                subspace_id,
                group_id,
                name,
                description,
                signer,
            }),
        }
        .into()
    }

    fn set_user_group_permissions(
        &self,
        subspace_id: u64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::SetUserGroupPermissions {
                subspace_id,
                group_id,
                permissions,
                signer,
            }),
        }
        .into()
    }

    fn delete_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::DeleteUserGroup {
                subspace_id,
                group_id,
                signer,
            }),
        }
        .into()
    }

    fn add_user_to_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::AddUserToUserGroup {
                subspace_id,
                group_id,
                user,
                signer,
            }),
        }
        .into()
    }

    fn remove_user_from_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::RemoveUserFromUserGroup {
                subspace_id,
                group_id,
                user,
                signer,
            }),
        }
        .into()
    }

    fn set_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> CosmosMsg<DesmosMsgRouter> {
        DesmosMsgRouter {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsg::Subspaces(SubspacesMsgs::SetUserPermissions {
                subspace_id,
                user,
                permissions,
                signer,
            }),
        }
        .into()
    }
}
