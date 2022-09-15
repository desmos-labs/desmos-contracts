use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::Rarity;

pub type RaritiesState = Vec<Rarity>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigState {
    pub admin: Addr,
    pub subspace_id: u64,
    pub cw721_code_id: u64,
}

pub const RARITIES: Item<RaritiesState> = Item::new("rarities");
pub const CONFIG: Item<ConfigState> = Item::new("config");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
