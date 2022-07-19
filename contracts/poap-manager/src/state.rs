use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_controllers::Admin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub poap_code_id: u64,
    pub subspace_id: u64,
    pub event_post_id: u64
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Admin = Admin::new("admin");
pub const POAP_ADDRESS: Item<Addr> = Item::new("poap_address");