use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct PendingTip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub max_pending_tips: u32,
}

pub type PendingTips = Vec<PendingTip>;

pub const MAX_CONFIGURABLE_PENDING_TIPS: u32 = 20u32;
pub const CONFIG: Item<Config> = Item::new("config");
pub const PENDING_TIPS: Map<(String, String), PendingTips> = Map::new("pending_tips");
