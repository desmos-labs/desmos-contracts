use cosmwasm_std::{Addr, Timestamp, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw721_code_id: Uint64,
    pub cw721_initiate_msg: Cw721InstantiateMsg,
    pub event_info: EventInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    EnableMint {},
    Mint {},
    MintTo {
        recipient: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventInfo {
    pub admin: Option<String>,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub per_address_limit: u32,
    pub base_poap_uri: String,
    pub event_uri: String,
    pub cw721_code_id: u64,
}