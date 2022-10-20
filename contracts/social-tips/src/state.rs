use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingTip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}

pub type PendingTips = Vec<PendingTip>;

pub const PENDING_TIPS: Map<(String, String), PendingTips> = Map::new("pending_tips");
