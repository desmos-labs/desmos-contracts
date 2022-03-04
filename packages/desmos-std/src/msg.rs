use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::msg::ProfilesMsg, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsg {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsgRoute,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMsgRoute {
    Profiles(ProfilesMsg),
}

impl Into<CosmosMsg<DesmosMsg>> for DesmosMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        CosmosMsg::Custom(self)
    }
}
impl CustomMsg for DesmosMsg {}

impl From<ProfilesMsg> for DesmosMsg {
    fn from(msg: ProfilesMsg) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsgRoute::Profiles(msg),
        }
    }
}
