use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::msg::Rarity;

pub type RaritiesState = Vec<Rarity>;

#[cw_serde]
pub struct ConfigState {
    pub admin: Addr,
    pub subspace_id: u64,
    pub cw721_code_id: u64,
}

pub const RARITIES: Item<RaritiesState> = Item::new("rarities");
pub const CONFIG: Item<ConfigState> = Item::new("config");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
pub const MINTED_TOKEN: Map<String, bool> = Map::new("minted_token");
