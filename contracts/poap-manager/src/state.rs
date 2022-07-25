use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub poap_code_id: u64,
    pub poap_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
