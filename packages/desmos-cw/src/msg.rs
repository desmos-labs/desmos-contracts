use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CosmosMsg, CustomMsg};
use crate::types::DesmosRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsgWrapper {
    pub route: DesmosRoute,
    pub msg: DesmosMsg,
}

impl Into<CosmosMsg<DesmosMsgWrapper>> for DesmosMsgWrapper {
    fn into(self) -> CosmosMsg<DesmosMsgWrapper> { CosmosMsg::Custom(self) }
}

impl CustomMsg for DesmosMsgWrapper {}
