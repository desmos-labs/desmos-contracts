use crate::{
    subspaces::{msg_router::SubspacesMsgRouter, msg_routes::SubspacesMsgs},
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

impl SubspacesMsgRouter<DesmosMsgRouter> for DesmosMsgRouter {
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

  
}
