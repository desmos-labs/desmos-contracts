use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RarityState {
    pub level: u32,
    pub mint_fees: Vec<Coin>,
    pub engagement_threshold: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigState {
    pub admin: Addr,
    pub subspace_id: u64,
    pub cw721_code_id: u64,
}

pub const RARITY: Map<u32, RarityState> = Map::new("rarity");
pub const CONFIG: Item<ConfigState> = Item::new("config");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
