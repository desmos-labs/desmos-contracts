use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub poap_code_id: u64,
}

pub const POAP_CONTRACT_ADDRESS: Item<Addr> = Item::new("poap_contract_address");

pub const CONFIG: Item<Config> = Item::new("config");
