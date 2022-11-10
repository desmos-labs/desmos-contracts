use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{index_list, IndexedMap, Item, MultiIndex};

#[cw_serde]
pub struct PendingTip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}

#[index_list(PendingTip)]
pub struct PendingTipsIndexes<'a> {
    pub sender: MultiIndex<'a, Addr, PendingTip, (String, String, Addr)>,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub max_pending_tips: u16,
    pub max_sent_pending_tips: u16,
}

pub const MAX_CONFIGURABLE_PENDING_TIPS: u16 = 20u16;
pub const MAX_CONFIGURABLE_SENT_PENDING_TIPS: u16 = 20u16;
pub const CONFIG: Item<Config> = Item::new("config");

pub fn pending_tips<'a>(
) -> IndexedMap<'a, (String, String, Addr), PendingTip, PendingTipsIndexes<'a>> {
    let indexes = PendingTipsIndexes {
        sender: MultiIndex::new(
            |_pk, data| data.sender.clone(),
            "pending_tips",
            "pending_tips__sender",
        ),
    };

    IndexedMap::new("pending_tips", indexes)
}
