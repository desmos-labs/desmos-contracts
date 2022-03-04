use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{subspaces::msg::SubspacesMsg, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsg {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsgRouter,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMsgRouter {
    Subspaces(SubspacesMsg),
}

impl Into<CosmosMsg<DesmosMsg>> for DesmosMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        CosmosMsg::Custom(self)
    }
}
impl CustomMsg for DesmosMsg {}

impl From<SubspacesMsg> for DesmosMsg {
    fn from(msg: SubspacesMsg) -> Self {
        Self {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsgRouter::Subspaces(msg),
        }
    }
}
