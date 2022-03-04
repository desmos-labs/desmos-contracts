use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::msg::ProfilesMsg, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgRouter {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMsg {
    Profiles(ProfilesMsg),
}

impl Into<CosmosMsg<DesmosMsgRouter>> for DesmosMsgRouter {
    fn into(self) -> CosmosMsg<DesmosMsgRouter> {
        CosmosMsg::Custom(self)
    }
}
impl CustomMsg for DesmosMsgRouter {}

impl From<ProfilesMsg> for DesmosMsgRouter {
    fn from(msg: ProfilesMsg) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsg::Profiles(msg),
        }
    }
}
