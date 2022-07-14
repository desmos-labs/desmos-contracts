use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub creator: Addr,
    pub admin: Addr,
    pub minter: Addr,
    pub mint_enabled: bool,
    pub per_address_limit: u32,
    pub cw721_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventInfo {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub base_poap_uri: String,
    pub event_uri: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const EVENT_INFO: Item<EventInfo> = Item::new("event_info");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
pub const NEXT_POAP_ID: Item<u64> = Item::new("nex_poap_id");
