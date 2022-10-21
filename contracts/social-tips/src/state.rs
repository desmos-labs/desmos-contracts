use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Map;

#[cw_serde]
pub struct PendingTip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}

pub type PendingTips = Vec<PendingTip>;

pub const PENDING_TIPS: Map<(String, String), PendingTips> = Map::new("pending_tips");
