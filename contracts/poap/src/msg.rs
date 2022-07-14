use cosmwasm_std::{Timestamp, Uint64};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    /// If `None` will be the address of who initialized the contract.
    pub admin: Option<String>,
    /// Address of who can call the [`ExecuteMsg::MintTo`] other then the admin.
    /// if `None` will be the address of who initialized the contract.
    pub minter: Option<String>,
    /// Id of the CW721 contract to initialize together with this contract.
    pub cw721_code_id: Uint64,
    /// Initialization message that will be sent to the CW721 contract.
    pub cw721_initiate_msg: Cw721InstantiateMsg,
    /// Information about the event.
    pub event_info: EventInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Admin command.
    EnableMint {},
    /// Admin command.
    DisableMint {},
    /// Can be called from any user, mint must be enabled.
    Mint {},
    /// Can be called from minter or admin, bypass mint enable flag.
    MintTo { recipient: String },
    /// Admin command.
    UpdateAdmin { new_admin: String },
    /// Admin command.
    UpdateMinter { new_minter: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventInfo {
    pub creator: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub per_address_limit: u32,
    pub base_poap_uri: String,
    pub event_uri: String,
    pub cw721_code_id: u64,
}
