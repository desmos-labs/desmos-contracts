use crate::msg::ServiceFee;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub subspace_id: u64,
    pub service_fee: ServiceFee,
    pub saved_tips_record_threshold: u32,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const TIPS_RECORD: Map<(u64, Addr, Addr), Vec<Coin>> = Map::new("tips_record");
